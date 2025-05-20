#!/bin/bash

# Validation of input -d
while getopts "d:" opt; do
  case $opt in
    d)
      directory="$OPTARG"
      ;;
    *)
      echo "Use: $0 -d /path/of/directory"
      exit 1
      ;;
  esac
done

# Validation Path
if [ -z "$directory" ]; then
  echo "❌ Send path of directory with param -d"
  echo "Ejemplo: ./build.sh -d /path/a/example.rust"
  exit 1
fi

# Obtener solo el nombre de la carpeta
name_dir=$(basename "$directory")

echo "🧩 Path: $directory"
echo "📁 Name Directory: $name_dir"

cd $directory
sudo cargo build --release
sudo cp ./target/release/"$name_dir" /usr/local/bin/"$name_dir"
export PATH="$HOME/.local/bin:$PATH"
