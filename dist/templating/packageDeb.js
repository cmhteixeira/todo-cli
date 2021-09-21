const { generateDebControl, properties } = require("./templating");
const fs = require('fs');
const fsP = require('fs').promises;
const path = require('path');
const { exec } = require("child_process");


let projectRoot = path.join(__dirname, "../../");
let releaseDir = path.join(projectRoot, "target/release");

// Test target/release folder exists (i.e. if cargo has built the binary)
let alreadyBuilt = fs.existsSync(releaseDir);

if (!alreadyBuilt) {
    console.log(`Directory: '${releaseDir}' doesn't exist. Most likely cargo hasn't build the sources yet. Run 'cargo build --release' at project root.`)
    process.exit(1);
}


// dpkg-deb creates a .deb file with the same name as the folder "it operates on"
let deb_file_name = `todo-cli_${properties.packageVersion}_amd64`;


let deleteStructureIfExists = fsP.rm(path.join(releaseDir, deb_file_name), { force: true, recursive: true })


let createExpectedStructure =
    deleteStructureIfExists
        .then(() => {
            return fsP.mkdir(path.join(releaseDir, deb_file_name), { recursive: true })
        })
        .then(() => {
            return fsP.mkdir(path.join(releaseDir, `${deb_file_name}/DEBIAN`), { recursive: true });
        })
        .then(() => {
            return fsP.mkdir(path.join(releaseDir, `${deb_file_name}/user/local/bin`), { recursive: true })
        });


// Generate control file based on the template
let controlDestP = createExpectedStructure
    .then(() => {
        return generateDebControl(path.join(releaseDir, `${deb_file_name}/DEBIAN`));
    });


// Make the binary smaller (Copied from here https://stackoverflow.com/questions/29008127/why-are-rust-executables-so-huge)
let stripExecutable = controlDestP.then(() => {
    let commandToCall = `strip ${path.join(releaseDir, properties.binaryName)}`;
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


let done = stripExecutable
    .then(() => {
        new Promise((resolve, reject) => {
            fs.copyFile(path.join(releaseDir, properties.binaryName), path.join(releaseDir, `${deb_file_name}/user/local/bin/${properties.binaryName}`), (err) => {
                if (err) reject(err);
                else resolve();
            })
        })
    });

// Run rpmbuild to _build_ the .rpm package
done.then(() => {
    let commandToCall = `dpkg-deb --build ${path.join(releaseDir, deb_file_name)}`;
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