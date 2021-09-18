#!/bin/bash

dir_of_script="$(dirname "$(realpath $0)")"
parent_dir="$(dirname "$(realpath $dir_of_script)")"
root_project_path="$(dirname "$(realpath $parent_dir)")"

source $dir_of_script/obtain_version.sh
version=$(versionFromTOML $root_project_path)


# dpkg-deb creates the .deb file with the name of the root folder
deb_file_name="todo-cli_${version}_amd64"

mkdir "$root_project_path"/target/release/$deb_file_name
root_debian="$root_project_path/target/release/$deb_file_name"

mkdir $root_debian/DEBIAN
mkdir $root_debian/usr
mkdir $root_debian/usr/local
mkdir $root_debian/usr/local/bin

# Generate the control file via templating
node $root_project_path/dist/templating/generateDebianControl.js

# Copy the generated control file
cp $dir_of_script/control $root_debian/DEBIAN

# Make the binary smaller (Copied from here https://stackoverflow.com/questions/29008127/why-are-rust-executables-so-huge)
strip $root_project_path/target/release/todo-cli

# Copy over the binary
cp $root_project_path/target/release/todo-cli $root_debian/usr/local/bin

# Make the debian file
dpkg-deb --build "$root_debian"

# Delete previously generated control file
rm $dir_of_script/control


