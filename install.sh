# Create the list
line=$(sed -n '3p' Cargo.toml)
members=$(echo $line | cut -d '=' -f 2 | tr -d '[' | tr -d ']')
IFS=',' read -ra member_list <<< "$members"

# Sort the list in descending order
IFS=$'\n' GLOBIGNORE='*' member_list=($(printf '%s\n' ${member_list[@]} | awk '{ print length($0) " " $0; }' | sort -n -r | cut -d ' ' -f 2-))

echo -ne "\033[0;32m"
for member in "${member_list[@]}"; do
  member_stripped="${member//\"/}"
  member_stripped=$(echo $member_stripped | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//')

  if [ "$member_stripped" != "strawberry" ]; then
    echo "[ ] Installing $member_stripped"
    cargo install --path $member_stripped -q "$@"

    if [ $? -eq 0 ]; then
      echo -e "\e[1A\e[K[x] Installed $member_stripped"
    fi
  fi
done
echo -ne "\033[0m"
