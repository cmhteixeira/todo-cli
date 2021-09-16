var { Liquid } = require('liquidjs');
const path = require("path")
const fs = require('fs');
const { getProjectName, getProjectVersion } = require('./obtainPackageInfo');


let distDir = path.join(__dirname, "../");
let propertiesFromJson = JSON.parse(fs.readFileSync(path.join(distDir, "properties.json")));
let cargoFile = path.join(__dirname, "../../Cargo.toml");;
let properties = { packageName: getProjectName(cargoFile), packageVersion: getProjectVersion(cargoFile), ...propertiesFromJson };

var engine = new Liquid();

engine
    .renderFile(path.join(distDir, "deb/control_template"), properties)
    .then((res) => {
        return fs.writeFileSync(path.join(distDir, "/deb/control"), res);
    });
