var toml = require('toml');
const fs = require('fs');


function readToml(cargoFile) {
    var contents = fs.readFileSync(cargoFile);
    var data = toml.parse(contents);
    return data;
}


function projectName(cargoToml) {
    return cargoToml.package.name;
}

function projectVersion(cargoToml) {
    return cargoToml.package.version;
}

function binaryName(cargoToml){
    return cargoToml.bin[0].name;
}

function getProjectName(cargoFile) {
    return projectName(readToml(cargoFile));
}

function getBinaryName(cargoFile) {
    return binaryName(readToml(cargoFile));
}


function getProjectVersion(cargoFile) {
    return projectVersion(readToml(cargoFile));
}

exports.getProjectName = getProjectName;
exports.getProjectVersion = getProjectVersion;
exports.getBinaryName = getBinaryName;