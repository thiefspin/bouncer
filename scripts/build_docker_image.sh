cd ./
version_string=$(cat Cargo.toml | grep version | head -1 | tr "=" "\n" | tail -1 | tr -d '"' | tr -d ' ')
echo building with version tag $version_string
CMD="docker build -t bouncer:$version_string ."
$CMD