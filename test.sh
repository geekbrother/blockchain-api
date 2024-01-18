if [ "true" != "true" ] && [ "latest" == "latest" ]; then
    echo "version="
  elif [ "latest" == "latest" ]; then
    echo "version=$(git tag | sort --version-sort | tail -n1)"
  else
    echo "version=latest"
  fi

