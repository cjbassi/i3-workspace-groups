# i3-workspace-groups

Allows for grouping of i3 workspaces for easier workspace management and navigation when working on seperate projects that each require multiple workspaces. Provides functions that add the ability to:

- move workspaces between groups
- focus a different group
- focus a different workspace in the current group

The name of the group is prepended to the name of each of its workspaces for clarity.  
Uses rofi to query for group names.

Port/rewrite of [infokiller/i3-workspace-groups](https://github.com/infokiller/i3-workspace-groups).

## Installation

Requires [rofi](https://github.com/DaveDavenport/rofi).

TODO Install from Cargo with:

```shell
cargo install i3-workspace-groups
```

TODO Install from the AUR with:

```
yay -S i3-workspace-groups
```

## Configuration

i3 config:

TODO: feel free to add your config file

[sxhkdrc](https://github.com/baskerville/sxhkd):

```
alt + {_, shift +} {0-9}
	i3-workspace-groups {focus-workspace,move-container-to-workspace} {10,1-9}

alt + ctrl + {_, shift +} {0-9}
	i3-msg {workspace number,move to workspace} {10,1-9}

alt + {_, shift +, ctrl + shift +} g
	i3-workspace-groups {focus-group,move-workspace-to-group,rename-group}
```

## Limitations

Workspace names can only be numbers.
Group names cannnot have a colon (`:`) in them.

## TODO

- testing
- comments
- previous-group command
- automatic workspace renaming
