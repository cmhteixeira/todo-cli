const fs = require('fs');
const fsP = require('fs').promises;
const path = require('path');
var tar = require('tar');
const { properties, generateRpmSpec } = require('./templating');
const { exec } = require("child_process");


let projectRoot = path.join(__dirname, "../../");
let releaseDir = path.join(projectRoot, "target/release");

// Test target/release folder exists (i.e. if cargo has built the binary)
let alreadyBuilt = fs.existsSync(releaseDir);

if (!alreadyBuilt) {
    console.log(`Directory: '${releaseDir}' doesn't exist. Most likely cargo hasn't build the sources yet. Run 'cargo build --release' at project root.`)
    process.exit(1);
}


let deleteStructureIfExists = fsP.rm(path.join(releaseDir, "rpmbuild"), { force: true, recursive: true })


let createExpectedStructure =
    deleteStructureIfExists
        .then(() => {
            return fsP.mkdir(path.join(releaseDir, "rpmbuild"), { recursive: true })
        })
        .then(() => {
            return fsP.mkdir(path.join(releaseDir, "rpmbuild/SOURCES"), { recursive: true });
        })
        .then(() => {
            return fsP.mkdir(path.join(releaseDir, "rpmbuild/SPECS"), { recursive: true })
        });


// Generate spec and file based on the template
// generateDebControl();
let specDestP = createExpectedStructure
    .then(() => {
        return generateRpmSpec(path.join(releaseDir, "rpmbuild/SPECS"));
    });


// Copy executable and tar-gz it to '%{topdir}/rpmbuild/SOURCES'
let done = specDestP
    .then(() => {
        new Promise((resolve, reject) => {
            fs.copyFile(path.join(releaseDir, properties.binaryName), path.join(releaseDir, `rpmbuild/SOURCES/${properties.binaryName}`), (err) => {
                if (err) reject(err);
                else resolve();
            })
        })
    })
    .then(() => {
        return fsP.mkdir(path.join(releaseDir, `rpmbuild/SOURCES/${properties.packageName}-${properties.packageVersion}`));
    })
    .then(() => {
        return fsP.copyFile(path.join(releaseDir, `rpmbuild/SOURCES/${properties.binaryName}`), path.join(releaseDir, `rpmbuild/SOURCES/${properties.packageName}-${properties.packageVersion}/${properties.binaryName}`))
    })
    .then(() => {
        return tar.c(
            {
                cwd: path.join(releaseDir, "rpmbuild/SOURCES"),
                gzip: true,
                file: path.join(releaseDir, `rpmbuild/SOURCES/${properties.packageName}-${properties.packageVersion}.tar.gz`)
            },
            [`${properties.packageName}-${properties.packageVersion}`]
        )
    });




// Run rpmbuild to _build_ the .rpm package
Promise.all([specDestP, done]).then(([specDest, as]) => {
    let commandToCall = `rpmbuild --define '_topdir ${path.join(releaseDir, "rpmbuild")}' -bb ${specDest}`;
    exec(commandToCall, (error, stdout, stderr) => {
        if (error) {
            console.log(`Error calling '${commandToCall}': ${error.message}`);
            return;
        }
        if (stderr) {
            console.log(`stderr for '${commandToCall}': ${stderr}`);
            return;
        }
        console.log(`Logs for '${commandToCall}': ${stdout}`);
    });
})