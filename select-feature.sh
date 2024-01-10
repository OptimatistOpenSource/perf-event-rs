#!/usr/bin/env bash

function is_version_leq() {
  if [[ $1 == $2 ]]; then
    echo 1
    return
  fi
  local IFS=.
  local i v1=($1) v2=($2)
  # fill empty fields in ver1 with zeros
  for ((i=${#v1[@]}; i<${#v2[@]}; i++)); do
    v1[i]=0
  done
  for ((i=0; i<${#v1[@]}; i++)); do
    if [[ -z ${v2[i]} ]]; then
        # fill empty fields in ver2 with zeros
        v2[i]=0
    fi
    if ((10#${v1[i]} > 10#${v2[i]})); then
        echo 0
        return
    fi
    if ((10#${v1[i]} < 10#${v2[i]})); then
        echo 1
        return
    fi
  done
  echo 1
}

declare -a versions=(
'6.3'
'6.0'
'5.16'
'5.13'
'5.12'
'5.11'
'5.9'
'5.8'
'5.7'
'5.5'
'5.4'
)

declare kernel_version=$1
declare kernel_major=$(echo $kernel_version | cut -d. -f1)
declare kernel_minor=$(echo $kernel_version | cut -d. -f2)

for version in "${versions[@]}"; do
  declare is_leq=$(is_version_leq $version "$kernel_major.$kernel_minor")
  if [[ $is_leq == 1 ]]; then
    echo "linux-$version"
    exit
  fi
done

echo "linux-unknown"
