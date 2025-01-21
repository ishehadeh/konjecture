#!/usr/bin/env bash
# bash boilerplate
readonly SCRIPT_NAME="$(basename "$0")"
readonly SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
function l { # Log a message to the terminal.
    echo
    echo -e "[$SCRIPT_NAME] ${1:-}"
}

# move to the root the notehub-js repo
cd "./ishehadeh.github.io"
echo "Open root of GH Pages repo"

# check if there's already a currently existing feature branch in notehub-js for this branch
# i.e. the altered openapi.yaml file's already been copied there at least once before
echo "Check if feature branch $BRANCH already exists in GH Pages"
git ls-remote --exit-code --heads origin $BRANCH >/dev/null 2>&1
EXIT_CODE=$?
echo "EXIT CODE $EXIT_CODE"

if [[ $EXIT_CODE == "0" ]]; then
  echo "Git branch '$BRANCH' exists in the remote repository"
  # fetch branches from notehub-js
  git fetch
  # stash currently copied openapi.yaml
  git stash
  # check out existing branch from notehub-js
  git checkout $BRANCH 
  # overwrite any previous openapi.yaml changes with current ones
  git checkout stash -- .
else
  echo "Git branch '$BRANCH' does not exist in the remote repository"
  # create a new branch in notehub-js 
  git checkout -b $BRANCH
fi

git add -A .
git config user.name github-actions
git config user.email github-actions@github.com
git commit -am "update konane"
git push --set-upstream origin "$BRANCH"

echo "Updated OpenAPI file successfully pushed to ishehadeh.github.io repo"