#!/bin/bash

# This function obtains the version of the library from the 'Cargo.toml' file
function versionFromTOML() {
  local pathFolder=$1

  local res=$(cat "$pathFolder"/Cargo.toml | sed -n "s/version = \"\(.*\)\"$/\1/p")
  echo $res
}