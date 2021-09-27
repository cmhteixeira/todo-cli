const { generateDebControl, properties } = require("./templating");
const fs = require('fs');
const fsP = require('fs').promises;
const path = require('path');
const { execSync } = require("child_process");


let projectRoot = path.join(__dirname, "../../");
let releaseDir = path.join(projectRoot, "target/x86_64-unknown-linux-musl/release");
let debFileName = `${properties.packageName}_${properties.debVersion}_amd64`; // dpkg-deb creates a .deb file with the same name as the folder "it operates on"
let debRoot = path.join(releaseDir, debFileName);
let debianFolder = path.join(debRoot, "DEBIAN");
let binaryDebFolder = path.join(debRoot, "usr/local/bin");
let binaryFile = path.join(releaseDir, properties.binaryName);


// Test target/x86_64-unknown-linux-musl/release folder exists (i.e. if cargo has built the binary)
let alreadyBuilt = fs.existsSync(releaseDir);

if (!alreadyBuilt) {
    console.log(`Directory: '${releaseDir}' doesn't exist. Most likely cargo hasn't build the sources yet. Run 'cargo build --release' at project root.`)
    process.exit(1);
}


let deleteStructureIfExists = fsP.rm(debRoot, { force: true, recursive: true })

let createExpectedStructure =
    deleteStructureIfExists
        .then(() => {
            return fsP.mkdir(debRoot, { recursive: true })
        })
        .then(() => {
            return fsP.mkdir(debianFolder, { recursive: true });
        })
        .then(() => {
            return fsP.mkdir(binaryDebFolder, { recursive: true });
        });


// Generate control file based on the template
let controlDestP = createExpectedStructure
    .then(() => {
        return generateDebControl(debianFolder);
    });


// Make the binary smaller (Copied from here https://stackoverflow.com/questions/29008127/why-are-rust-executables-so-huge)
let stripExecutable = controlDestP.then(() => {
    execSync(`strip ${binaryFile}`);
})


let done = stripExecutable
    .then(() => {
        new Promise((resolve, reject) => {
            fs.copyFile(
                binaryFile,
                path.join(binaryDebFolder, properties.binaryName),
                (err) => {
                    if (err) reject(err);
                    else resolve();
                })
        })
    });

// Run rpmbuild to _build_ the .rpm package
let final = done.then(() => {
    execSync(`dpkg-deb --build ${debRoot}`);
});
