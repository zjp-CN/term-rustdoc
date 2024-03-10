# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### 🚀 Features

- Add ItemInnerKind for Navi
- Render NaviOutline
- Update_outline from NaviOutline for InnerItem
- Highlight_current_line on NaviOutline
- Impl_tree on DModule for displaying ITABImpls
- Always able to jump back to module tree
- DImplInner; sort by name in DImpl and DImplInner
- Merge inherent items
- Sort associated items by name in Traits
- Don't restore terminal when error happens
- Recompile outdated doc cache when opening the doc
- Tab keystroke for switching between NaviAction
- NaviAction::TraitAssociated and TraitImplementors tree view
- Set_previous_action on Left or h keystroke
- Jkl keystroke alias
- Display fields_tree for NaviAction::StructInner
- Display fields_tree for NaviAction::StructInner on Union

### 🐛 Bug Fixes

- Remove trailing whitespace in rendering markdown lines
- Get_item_inner by searching in recursive modules
- NaviOutline kind for reexport items
- Use visible_lines in get_line_on_screen
- Render width and max_width for Outline
- Show implementor on inherent block
- DImplInner show fn; update snapshots
- Add NaviAction::Item; fix navi interaction bugs
- Pin the width of navi
- NaviOutline and cursor behavior
- Reset cursor position when init setu

### 🚜 Refactor

- Mv Page to own module
- Rm CrateDoc from NaviOutline; add Scroll area for it
- Outline with OutlineInner
- Mv DataItemKind to lib; search after the kind is known

### 📚 Documentation

- Add github release badget
- Update Help for outline

### 🧪 Testing

- Update snapshots

### ⚙️ Miscellaneous Tasks

- Mv Navi to own module
- Don't make fields in Navigation public
- NaviOutline owns CrateDoc, thus only need to pass selected ID
- Set_item_inner return Option<ItemInnerKind>
- Add inner_item for Outline
- Add render: OutlineKind in Outline
- ScrollTreeLines for NaviOutline
- Render NaviOutline
- Leave enough space for NaviOutline
- Compute height and width for NaviOutline
- Use NaviAction to pass action to Outline from Navi
- Reset_navi_outline; back_to_home
- Default for DocTree
- Log path for empty InnerItem TreeLines
- Rm doc arg for rendering inner_item
- Rename InnerItem in Outline to Setu
- Const first in display order, then type alias, finally fn
- Warn bad state with names in DTrait::new
- Slim icons for associated items
- Adjust TOC width; add right padding for md
- Help md padding
- Display fields_tree without node tag
- Names_node @iter case & doc; expand/fold only for Module tree

## [0.1.1] - 2024-03-02

### 🚀 Features

- Cargo dist init for releasing binaries

### 🐛 Bug Fixes

- Logger file should differ on dev/release

### 📚 Documentation

- Add license and version badgets

## [0.1.0] - 2024-03-02

### 🚀 Features

- DModule extracts the local items as structural view on APIs
- TotolCount for DModule by counting items
- ImplCounts for counting inherent/trait_ and total impl blocks
- Impl Show for DModule and its components as Tree display
- Query path in show_prettier
- Pub use DMacro*
- Add IDMap::name and show_names based on short names from IndexMap
- Don't display the empty items; simplify code by names_node!
- Tag for each line/node
- TreeLines as the equivalent of DocTree
- Page widget and display Outline
- More colors on Tags/nodes
- ScrollOffset and HOME/END/PageUp/PageDown control
- Highlight the current cursor in outline
- Mv cursor by one line; fix cursor position overflow
- Add max_width field in Scrollable
- File logger
- Debug for Page; mv Scrollable to own module
- Move_{bottom|top|middle}_cursor and vim-like keybind L/H/M
- Make Scrollable generic over each line; expose type alias
- WIP parse markdown with highlights
- Rc<RustDoc> as CrateDoc; use it in Page.Content
- Update_doc for Content; widget for ScrollText
- Border for Page's components
- Set_current_component by setting bg on current Component
- Scroll on content page 🎇
- Item_tree for DModule and as the default view for initialization
- Support Import (pub use item)
- Sort_by_name on DModule 🎇
- Expand_current_module_only for TreeLines (and UI)
- Check_if_can_return_to_previous_cursor after scrolling and folding
- Small UX improvement by reducing cursor jump when scrolling half page
- Support mouse click on outline to update doc content display
- Expand_toggle for toggling module nodes
- Expand_zero_level and default to it
- WIP parse markdown and cache contents by words/lines/blocks
- StyledText with Interaction
- <spcace> shortcut for toggle_sytect in content panel
- Respect max_width of content when wrapping texts
- Display referees via [text][number] and links via [number]: url
- Double click on outline_fold_expand_toggle
- WIP ScrollHeading in Navigation
- Display Navigation of headings
- Jump to a heading
- Highlight current headings in visual range
- All_pkgs_in_latest_registry & lastest_pkgs_in_latest_registry
- Add DashBoard and UI; add LocalRegistry and Frame
- Display and scroll local_registry_pkgs
- Fuzzy match for local registry pkgs
- Show match result counts on the bottom right of border
- Show number ordering before each pkg name in the list
- Display version after pkg name
- Scroll_home/end for registry; fix starting position in clear pat
- Highlight current line in registry pkg
- Move_forward/backward_cursor; fix cursor behavior
- WIP add DataBase CachedDocInfo PackageDoc; serde for CrateDoc
- Build doc by user's selection, and cache it to redb
- Cache PkgInfo; extract encode_to_vec as encode
- Use index.db to store CachedDocInfo list
- WIP add DataBaseUI
- Read all_caches from DataBase
- Display cached doc of pkg & version
- Display Cache kind
- Tab to switch_panel; highlight current line in database; load_doc
- Only hightlight the current in current panel
- Update_in_progress in rendering
- Use xz to compress raw data before saved into db file
- Support multiple ways to sort caches and display the sort desc
- Display counts on caches' states; hide sort desc if narrow
- Load doc for Page; switch focus in Frame
- Delete key for downgrading from loaded to cached
- Fuzzy search for DataBase and Both source
- Border title for dashboard search
- Show data structures after other items
- Handle mouse event when in DashBoard
- Double_click for DashBoard
- Quick switch to Page via mouse click on DashBoard
- Help Popup 🎇
- Env logger filter like RUST_LOG=term_rustdoc::tree::nodes=debug
- Features selection
- Reuse feature selection if on the same pkg
- Skip feature selection if there is only default for nothing
- Add version selection in features selection panel
- Mv version selection to left of feature selection
- Selection Panel switch via click or Tab; Esc for close
- PkgToml line under Database/Registry panels
- PkgToml line under Features/Versions Selection

### 🐛 Bug Fixes

- Make DTypeAlias as a simple wrapper type
- Don't lose item name when its inner is is_empty
- Contain_private_fields info in DStruct::show_prettier
- Tag colors; add Tag::Implementations
- Start pos in scrollup_outline when hitting the bottom
- Warn about outline width
- Emit Unknown tag for undesired calling on Tag::show
- No bg on content; don't cross width
- Scrollable interaction for generic `Lines: Deref<Target = [T]>`
- Non-cjk width for TreeLine to reduce outline width to minimum
- Tag colors
- Default bg block highlight to outline
- Default scrolling to outline Component
- Jump to previous line when nothing shows up after folding
- Node lost in expand_first_level_modules_only
- Rewrite CurrentModule behavior; refactor fold::Kind
- Unify the module vs other items order by putting modules to last
- Expand_current_module_only on root should have zero level items
- Forbid toggling in CurrentModule
- Heading iterator
- Respect trailling_whitespace when converting Word to StyledText
- Don't write a new line when there is no link/footnote in a block
- Intra_code in links
- Update_doc when screen size changes; clean up outdated code
- Parse Heading via parse_paragraph
- Add sharps symbols back and set styles for heading blocks
- Skip the first empty word in the line start
- Intra_code in links reuses LINK style
- Push_link and push_footnote for Block
- Parse_{emphasis,strong,strike_through} via parse_nested_styles
- Add the key back when displaying footnotes
- Split a long link to multiple lines to avoid text from being hidden
- Don't change cursor postion in outline when hitting out of range
- Update_search should reset starting position
- Load only if loadable; log loading time
- Recover error from TableDoesNotExist for CachedDocInfo
- Don't duplicate caches item when it's cahced before
- Level up BeingCached in sorting: Loaded - BeingCached - Unloaded
- Loadable for LoadedDoc
- Enforce started time in initialization
- Filter ignored rust code starting with optional whitespaces in md
- Nested codeblock in list items
- Codeblock in entry point as nested codeblocks do
- Help interaction with DashBoard/Page; fix other interaction bugs
- Check double click range in range before any opration
- Render_line when it's still available to stopping text
- Still push external item ID as normal item, but won't show doc
- Sort by Version instead of str in get_all_version
- Update_pkg_toml for switch_panel & respond_to_char

### 🚜 Refactor

- Split tree::DModule into smaller components in tree/nodes/*
- Mv src/main.rs to src/bin/main.rs
- Add DModule tree for folding and tree traversal
- Mv CrateDoc to term_rustdoc lib
- Mv DModule into CrateDoc to share Rc
- IDMap is the owned data with ID buffer behind CrateDoc's Rc
- Impl Deref<IDMap> for CrateDoc
- Replace ID with CompactString wrapper type
- Rm Line generic parameter in Scrollable by associated type
- Define ColumnSpan in StyledText; add width in StyledLine; add Regions
- WriteLines
- LinkedRegions
- Words_to_line
- Add heading in Links, merge_continuous in TargetRegion
- Add LinkedRegions in StyledLines; simplify write_styled_lines
- Mv heading's TargetRegion to Links
- Headings carries SelectedRegion rather than TargetRegion
- Rm last_click field; use integration.json on Windows
- Replace Nucleo by custom shared Fuzzy wrapper
- Find the pkg's Cargo.toml via Path::exists instead of WalkDir
- Mv database module to under DashBoard
- Build doc; add Duration in DocMeta; mv DocMeta to CachedDocInfo
- Mv DataBase to under DataBaseUI; Cache's sorting order
- Replace in_progress fieldb by event sender
- Use empty_state to prevent ownship taking for CacheInner
- Make Scrollable a trait object by Scroll trait

### 📚 Documentation

- Update usage on names_node!
- Names_node! update
- Add Roadmap in README
- Add feature support on items in roadmap
- Update outline folding in README
- Update Roadmap in README
- Update README
- Mention `Ctrl-w` and `Ctrl-q`; use Gif pic for help
- Add outline design draft
- Add Navigation Panel design
- Update Feature selection popup
- Document the help of Ctrl-c in search panel

### 🧪 Testing

- Rename integration items; update snapshots
- Add more items for snapshots
- Update snapshots
- Update snapshots because reexported items are supported
- Update snapshots because module nodes are put to last in tree
- Add tests of generate_doc_json for tokio and integration
- Update snapshots due to no trailing whitespace in intra code
- Add LinkedRegions snapshot
- Update snapshots due to update in nightly rustc
- Update snapshots
- Add registry_path & tokio_path
- Update ignore reason
- Add quick doc compilation test for future use

### ⚙️ Miscellaneous Tasks

- Earyly ID conversion; pub new
- Imagine a deeper design about counting in impls
- Mv trivial trait impls to submods (like Debug/Display)
- Replace verbose Show-Dmodule impl by impl_show!
- Update snapshots; fix docs
- Fmt
- Clean up dead code
- Mv Tag to its own module
- Mv Text* to own module
- Clean up icon! in show; doc
- Split widget rendering of Scrollable to module
- Mv scroll/cursor interaction on outline to module
- ScrollTreeLines alias for Scrollable<TreeLines>
- Revert new on Scrollable
- &Block implements Widget already
- IDMap::from_crate -> IDMap::new
- Tag color
- Bg color for current block
- Clean up
- ╰ as last_item in folding tree
- Rename to expand_to_first_level_modules
- Mv ScrollText to fallback; clean up dead imports path
- Separate Blocks/Links Block Line to own modules
- Set content wrapping width to 80
- Add parse_markdown_dbg for quick testing
- Allow dead_code in integration package
- Doc; clean up
- Mute unused warning for TargetRegion
- Init Page with terminal size
- Kepmap d for toggle_sytect; unmap space key
- Mv border from content to outline; rm panic; change SET color
- Doc; rm gap_style from render_line_fill_gap
- Rename Component to Panel
- Split Panel to own module
- Mv layout to own module
- Log for setup and rendering
- Set double_click timeout to 450ms; log for init
- Mv Search to own module
- <C-c> for clearing input; <C-q> for quitting; rm Injector field
- Fuzzy field should be wrapped in Option
- Rm nucleo dep
- Rm dead code
- Mv local_registry under root
- Mv PkgKey to own module
- Rm render_line_fill_gap reexport
- Mv CachedDocInfo into own module
- Mv DocMeta to own module
- Mv build, encode, decode fn to util module
- Early return when nothing needs to display in registry
- Allow ansi color in logging
- Switch to database if caches are not empty
- Early return None when empty in visible_line on Scrollable
- Improve new fn for Content
- Split less tight Cache code into modules
- Uppercase in border titles
- Mv update_for_key to submod
- Update Cargo.toml
- Use the new get_line_on_screen API for heading_jump
- Rm App structure to slim codebase
- Use stable intersperse; refactor due to unused lints removal
- Downgrade Page rendering log level
- Display traits before structs instead of first; show fns first
- Downgrade logging level to info
- Define common colors/styles
- Mv Cache kind style to colors module
- Mv cargo_toml into spawn in build fn
- Rm verbose Features words
- Rename Scrollable struct to Scroll, Scroll trait to Scrollable
- Exclude outline-design.txt from pkg

### ◀️ Revert

- Rm L on Scrollable; use Deref<Target=[L]> instead AsRef<[T]> as the bound

<!-- generated by git-cliff -->
