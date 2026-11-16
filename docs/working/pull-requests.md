# Commits, pull requests, merges and releases

Read before committing, branching, opening or updating a PR, merging, tagging, or posting on GitHub.

## Branches and commits

- Every change, docs included, is made on its own named branch (`t3/<topic>`), never on `master`.
  Never push to `master` unless the user asks in that request; an earlier yes doesn't carry over.
- Commit finished work, with a message that says what changed and why, plus the harness's
  attribution trailer.
- This repo has no remote yet. Don't add one or push unless the user asks. The original repos
  (`quicksnip`, benjkr/quicktooth) are left as they are: never push to them from here.

## Pull requests (once there's a remote)

- "Make PR" means: push the branch, open the PR from `.github/pull_request_template.md`, and link it
  in T3 (`link_pull_request`).
- The description shows the latest state: after a push that changes what the PR does or looks
  like, rewrite it and update its media. Visible changes carry the blade's preview media (a GIF or
  stills, before/after when it replaces something).
- Stacked PRs: base each on the branch below it. After the lower one merges, retarget the next
  (`gh pr edit <n> --base master`), check it's mergeable, then merge.
- A comment posted on GitHub by an agent goes out under the user's account, so it starts by saying
  it's Claude, commenting on their behalf.

## Merging

- Only when the user asks. Use merge commits, one PR at a time, in stack order. Then fast-forward
  the local `master`.
- After merging, nothing gets installed automatically. The user's PC runs only `master`, from a
  tag once releases are tagged ([live testing](live-testing.md)).

## Releases (when the user wants one)

- Each blade is released on its own: tag `master` as `<blade>-vX.Y.Z` (`quicktooth-v1.3.0`) and
  build from the tag. The blade's build spec says where its version lives and what must be true
  before anything public.
