var { Liquid } = require('liquidjs');
const path = require("path")
const fs = require('fs');
const fsP = require('fs').promises;
const { getProjectName, getProjectVersion, getBinaryName } = require('./obtainPackageInfo');


let distDir = path.join(__dirname, "../");
let propertiesFromJson = JSON.parse(fs.readFileSync(path.join(distDir, "properties.json")));
let cargoFile = path.join(__dirname, "../../Cargo.toml");;
let properties = { 
    ...propertiesFromJson,
     packageName: getProjectName(cargoFile),
    packageVersion: getProjectVersion(cargoFile), 
    binaryName: getBinaryName(cargoFile)
};

var engine = new Liquid();

let generateDebControl = (destFolder) => {
    let destGeneratedControl = path.join(destFolder, "control");
    return engine
        .renderFile(path.join(distDir, "deb/control_template"), properties)
        .then((res) => {
            return fsP.writeFile(destGeneratedControl, res);
        });
}


let generateRpmSpec = (destFolder) => {
    let destGeneratedRpm = path.join(destFolder, `${properties.packageName}.spec`);
    return engine
        .renderFile(path.join(distDir, "rpm/spec-template"), properties)
        .then((res) => {
            return new Promise((resolve, reject) => {
                fs.writeFile(destGeneratedRpm, res, (err) => {
                    if (err) reject(err);
                    else resolve(destGeneratedRpm);
                });
            })
        });
}


exports.generateDebControl = generateDebControl;
exports.generateRpmSpec = generateRpmSpec;
exports.properties = properties;