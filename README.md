# i3-workspace-groups

Adds the ability to group i3 (and Sway) workspaces for easier workspace management and navigation when working on several projects that independently require multiple workspaces. Provides functions that add the ability to:

- move workspaces between groups
- focus a different group
- focus a different workspace in the current group

The name of the group is prepended to the name of each of its workspaces for clarity.

Uses Rofi to query for group names.

Port/rewrite of [infokiller/i3-workspace-groups](https://github.com/infokiller/i3-workspace-groups).

## Installation

Requires [Rofi](https://github.com/DaveDavenport/rofi).

Install from Cargo with:

```shell
cargo install i3-workspace-groups
```

<!-- TODO Install from the AUR with: -->

<!-- ```
yay -S i3-workspace-groups
``` -->

## Configuration

### Keybinds

i3 config:

```
bindsym $mod+1 exec i3-workspace-groups focus-workspace 1
bindsym $mod+2 exec i3-workspace-groups focus-workspace 2
bindsym $mod+3 exec i3-workspace-groups focus-workspace 3
bindsym $mod+4 exec i3-workspace-groups focus-workspace 4
bindsym $mod+5 exec i3-workspace-groups focus-workspace 5
bindsym $mod+6 exec i3-workspace-groups focus-workspace 6
bindsym $mod+7 exec i3-workspace-groups focus-workspace 7
bindsym $mod+8 exec i3-workspace-groups focus-workspace 8
bindsym $mod+9 exec i3-workspace-groups focus-workspace 9
bindsym $mod+0 exec i3-workspace-groups focus-workspace 10

bindsym $mod+Shift+1 exec i3-workspace-groups move-container-to-workspace 1
bindsym $mod+Shift+2 exec i3-workspace-groups move-container-to-workspace 2
bindsym $mod+Shift+3 exec i3-workspace-groups move-container-to-workspace 3
bindsym $mod+Shift+4 exec i3-workspace-groups move-container-to-workspace 4
bindsym $mod+Shift+5 exec i3-workspace-groups move-container-to-workspace 5
bindsym $mod+Shift+6 exec i3-workspace-groups move-container-to-workspace 6
bindsym $mod+Shift+7 exec i3-workspace-groups move-container-to-workspace 7
bindsym $mod+Shift+8 exec i3-workspace-groups move-container-to-workspace 8
bindsym $mod+Shift+9 exec i3-workspace-groups move-container-to-workspace 9
bindsym $mod+Shift+0 exec i3-workspace-groups move-container-to-workspace 10

bindsym $mod+Control+1 workspace number 1
bindsym $mod+Control+2 workspace number 2
bindsym $mod+Control+3 workspace number 3
bindsym $mod+Control+4 workspace number 4
bindsym $mod+Control+5 workspace number 5
bindsym $mod+Control+6 workspace number 6
bindsym $mod+Control+7 workspace number 7
bindsym $mod+Control+8 workspace number 8
bindsym $mod+Control+9 workspace number 9
bindsym $mod+Control+0 workspace number 10

bindsym $mod+Shift+Control+1 move container to workspace number 1
bindsym $mod+Shift+Control+2 move container to workspace number 2
bindsym $mod+Shift+Control+3 move container to workspace number 3
bindsym $mod+Shift+Control+4 move container to workspace number 4
bindsym $mod+Shift+Control+5 move container to workspace number 5
bindsym $mod+Shift+Control+6 move container to workspace number 6
bindsym $mod+Shift+Control+7 move container to workspace number 7
bindsym $mod+Shift+Control+8 move container to workspace number 8
bindsym $mod+Shift+Control+9 move container to workspace number 9
bindsym $mod+Shift+Control+0 move container to workspace number 10

bindsym $mod+g exec i3-workspace-groups focus-group
bindsym $mod+Shift+g exec i3-workspace-groups move-container-to-group
bindsym $mod+Shift+Control+g exec i3-workspace-groups rename-group
```

[sxhkd](https://github.com/baskerville/sxhkd):

```
alt + {_, shift +} {0-9}
	i3-workspace-groups {focus-workspace,move-container-to-workspace} {10,1-9}

alt + ctrl + {_, shift +} {0-9}
	i3-msg {workspace number,move to workspace number} {10,1-9}

alt + {_, shift +, ctrl + shift +} g
	i3-workspace-groups {focus-group,move-container-to-group,rename-group}
```

### Status bar

i3status:

```
bar {
    strip_workspace_numbers yes
}
```

[polybar](https://github.com/jaagr/polybar):

```dosini
[module/i3]
type = internal/i3

strip-wsnumbers = true
```

[waybar](https://github.com/Alexays/Waybar):

```
"sway/workspaces": {
    "format": "{name}"
}
```

## Limitations

- Workspaces in groups cannot have custom names (i.e. they have to remain their default number).
  - The default workspaces that are not in a group can have custom names, but have to have a preceded by a number.
- Group names cannnot have a colon (`:`) in them.
