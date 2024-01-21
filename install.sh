line=$(sed -n '3p' Cargo.toml)
members=$(echo $line | cut -d '=' -f 2 | tr -d '[' | tr -d ']')
IFS=',' read -ra member_list <<< "$members"

for member in "${member_list[@]}"; do
  member_stripped="${member//\"/}"
  member_stripped=$(echo $member_stripped | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//')

  cargo install --path $member_stripped
done
