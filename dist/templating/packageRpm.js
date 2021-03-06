const fs = require('fs');
const fsP = require('fs').promises;
const path = require('path');
var tar = require('tar');
const { properties, generateRpmSpec } = require('./templating');
const { execSync } = require("child_process");


let projectRoot = path.join(__dirname, "../../");
let releaseDir = path.join(projectRoot, "target/x86_64-unknown-linux-musl/release");
let rpmRoot = path.join(releaseDir, "rpmbuild");
let specsFolder = path.join(rpmRoot, "SPECS");
let sourcesFolder = path.join(rpmRoot, "SOURCES");
let binaryHolderFolder = path.join(sourcesFolder, `${properties.packageName}-${properties.rpmVersion}`);
let binaryFile = path.join(releaseDir, properties.binaryName);

// Test target/x86_64-unknown-linux-musl/release folder exists (i.e. if cargo has built the binary)
let alreadyBuilt = fs.existsSync(releaseDir);

if (!alreadyBuilt) {
    console.log(`Directory: '${releaseDir}' doesn't exist. Most likely cargo hasn't build the sources yet. Run 'cargo build --release' at project root.`)
    process.exit(1);
}


let deleteStructureIfExists = fsP.rm(rpmRoot, { force: true, recursive: true });


let createExpectedStructure =
    deleteStructureIfExists
        .then(() => {
            return fsP.mkdir(rpmRoot, { recursive: true });
        })
        .then(() => {
            return fsP.mkdir(sourcesFolder, { recursive: true });
        })
        .then(() => {
            return fsP.mkdir(specsFolder, { recursive: true });
        });


// Generate spec and file based on the template
let specDestP = createExpectedStructure
    .then(() => {
        return generateRpmSpec(specsFolder);
    });


// Make the binary smaller (Copied from here https://stackoverflow.com/questions/29008127/why-are-rust-executables-so-huge)
let stripExecutable = specDestP.then(() => {
    execSync(`strip ${binaryFile}`);
})



// Copy executable and tar-gz it to '%{topdir}/rpmbuild/SOURCES'
let done = stripExecutable
    .then(() => {
        new Promise((resolve, reject) => {
            fs.copyFile(
                binaryFile,
                path.join(sourcesFolder, properties.binaryName),
                (err) => {
                    if (err) reject(err);
                    else resolve();
                })
        })
    })
    .then(() => {
        return fsP.mkdir(binaryHolderFolder);
    })
    .then(() => {
        return fsP.copyFile(
            path.join(sourcesFolder, properties.binaryName),
            path.join(binaryHolderFolder, properties.binaryName)
        )
    })
    .then(() => {
        return tar.c(
            {
                cwd: sourcesFolder,
                gzip: true,
                file: path.join(sourcesFolder, `${properties.packageName}-${properties.rpmVersion}.tar.gz`)
            },
            [`${properties.packageName}-${properties.rpmVersion}`]
        )
    });


// Run rpmbuild to _build_ the .rpm package
Promise.all([specDestP, done]).then(([specDest, as]) => {
    execSync(`rpmbuild --define '_topdir ${rpmRoot}' -bb ${specDest}`);
})