# Upset
_Upset is a command line tool to setup any computer quickly using a YAML configuration file_

---

Are you upset every time you need to re-install your computer again? We'll I am, that's why I created this tool to setup
any computer quickly using a simple YAML-file.

## Basic usage

```bash
Usage: upset --configuration-file <CONFIGURATION_FILE>

Options:
    -c, --configuration-file <CONFIGURATION_FILE>
```

```bash
$ upset -c ./setup.yaml

  ✓ Successfully installed git.git
  ⚠ Unable to install upset
```

## Example configuration

### Example 1

_Installing packages through WinGet and cloning some repositories_

```yaml
version: 1.0
configuration:
  downloads:
  packages:
    - package_manager: winget
      source: winget
      applications:
        - git.git
        - vscode
    - package_manager: winget
      source: msstore
      applications:
        - Jetbrains.Idea.Community
  version_control:
    - vcs: git
      destination_folder: ~/Git-repos
      repositories:
        - git@github.com:bartkessels/it-depends
        - git@github.com:bartkessels/upset
```

### Example 2

_Installing packages but not cloning repositories_

```yaml
version: 1.0
configuration:
  downloads:
  packages:
    - package_manager: winget
      source: winget
      applications:
        - git.git
        - vscode
    - package_manager: winget
      source: msstore
      applications:
        - Jetbrains.Idea.Community
  version_control:
```

### Example 3

_Cloning repositories but not installing packages_

```yaml
version: 1.0
configuration:
  downloads:
  packages:
  version_control:
    - vcs: git
      destination_folder: ~/Git-repos
      repositories:
        - git@github.com:bartkessels/it-depends
        - git@github.com:bartkessels/upset
```