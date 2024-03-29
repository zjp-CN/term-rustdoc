Use `Ctrl-q` to quit :) Just kidding.

# Cursor

Current cursor is highlighted and controlled by a key press or left click.

* `Up` or `Down` moves the cursor up/down a line.
* left click moves the cursor to any line in visual range

# Scrolling

Areas allow scrolling by mouse or key presses like `PageUp` / `PageDown` / `Home` / `End`.

For cursor behavior in scrolling:
* the cursor tries to jump to a previous line for the same content
* if the jump is not viable, it will move or stay as per scrolling

# Dash Board

## Search

Press `Ctrl-f` to switch between these:
* fuzzy search pkgs in local registry panel
* fuzzy search docs in database panel
* fuzzy search in both local registry and database panels

Press `Ctrl-c` to clear out the input.

## DataBase

> **NOTE: to switch between database and registry panel, use `Tab` key press.**

### KeyMap

* `Enter`: load a cached doc and enter the Doc Page.
* `Delete`: unload a doc, i.e. the Loaded doc downgrades to Cached.

### Mouse

* Double click on the cursor item: same as `Enter` key press to load a doc.
* Right click on the cursor item: same as `Delete` key press to unload a doc.

## Registry

Local registry provides the source packages, and docs are generated by these pkgs.

It lies in your `~/.cargo/registry/src/` on Linux system, for example.

### KeyMap

* `Enter`: pop up feature selection for selected pkg.

## Selection

Select a version with features to compile.

Mouse click is supported:
* left click in range: choose/switch between Features and Version panel.
* right click out of range: back to Registry panel.

KeyMap:

* `Space`: compile doc with selected features.
* `Tab`: switch between Features and Version panel.
* `Esc`: close Selection popup, and return to Registry

### Features

Feature selection is an interactive panel to select features to compile doc with.

Features that are checked ( ) or emit warnings ( ) will be passed to compilation.

There are four signs:
* no sign: not selected and not passed to doc compilation
*  : selected and passed
* 🔒: not selected, but implies the feature will be enabled by other features,
      thus you don't need to enable it.
*  : selected and passed, but implies the feature has already been enabled by 
      other features, thus you don't need to enable it.

Extra KeyMaps:

* `Enter`: toggle a feature. (same as double left click)

> **NOTE: there is no way to cancel once doc compilation starts.**

### Version

Single left click to choose a version, then Features candidates will be updated.

# Doc Page

> **NOTE: meaningless click in DashBoard can switch to Page.**

> **NOTE: Use `Ctrl-w` in both Page and DashBoard to switch between them!**

From left to right, there are *outline* panel, *content* panel and *navi* panel.

## Outline

### Navi Action

Navigation action displayed on right bottom will replace module tree by a detail
tree into the inner of an item from module tree:
* `Tab` or `l` or `Right` arrow key: next action
* `h` or `Left` arrow key: previous action

These actions are:
* for struct/union under cursor, fields and impls
* for enum under cursor, variants and impls
* for trait under cursor, associated items and implementors
* for module under cursor, make the module node as new root with items tree expanded.
  This is very immature.

### Module Tree

Control the outline module tree nodes by folding/expansion:
* `Enter`: expand/fold a single node.
* `/`: expand all nodes.
* `0`: only expand zero level nodes that directly under the root node.
     This means modules under root will be folded.
* `1`: expand zero and first level nodes that under the root node till the first depth.
     This means modules under first-level modules will be folded.
* `m`: only expand the current module including nested one in it, but with other modules
     that doesn't share the same ancestor from root folded.

Some keymaps to control cursor position like Vim:
* `L`: move the cursor to bottom node in current view range.
* `H`: move the cursor to top line in current view range.
* `M`: move the cursor to middle line in current view range.
* `j`: alias for `Down` arrow key for moving the cursor down a line
* `k`: alias for `Up` arrow key for moving the cursor down a line

### Mouse

* Double click: same as `Enter` key press to expand or fold a node.
* Left click: select a tree node and display the markdown doc in content panel.

## Content 

Aside from the scrolling control, there is one `d` keymap to toggle the markdown rendering.

Since this program uses custom parsing to hightlight markdown and wrap texts, we only use
`syntect` crate to hightlight syntaxes in codeblocks.

If you want the original markdown content from raw json docs, `d` key press will switch
to render them for you with hightlighting from `syntect`.

# TOC

Markdown content can be scrollable with TOC on the right!

TOC is featured as
* auto-updated: it always hightlight the headings in visual range.
* clickable: it jumps to a place where the heading is moved to the top.

