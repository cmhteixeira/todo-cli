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

function getProjectName(cargoFile) {
    return projectName(readToml(cargoFile));
}


function getProjectVersion(cargoFile) {
    return projectVersion(readToml(cargoFile));
}

exports.getProjectName = getProjectName;
exports.getProjectVersion = getProjectVersion;