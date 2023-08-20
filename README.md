## Tarkov stash

Simple stash editor with a simple UI. If you need more advanced features I recommend to use [SPT-API Profile editor](https://hub.sp-tarkov.com/files/file/184-spt-aki-profile-editor/)

This is based out of another project of mine: [task-stash-console](https://github.com/angel-git/tarkov-stash-console):

### Features

- Backup your profile
- Set _found in raid_ to items
- Increase stock of items (currency, ammo)
- Restore durability, usage etc from armors, keys, meds...

### Limitations

- Some images are not correct as they don't include all attachments
- Rotated items don't look nice

### Future features

- Compatible with 0.3.15 (v2)
- Duplicate items (v2)

### Screenshots

TODO

### Download

> Backup your profile before running this.

Never run any `.exe` file that you have downloaded from internet. Including this. Use it at your own risk.

See the [releases](https://github.com/angel-git/tarkov-stash/releases) page.

### Development

See the current roadmap for v1: https://github.com/users/angel-git/projects/1/views/3

#### Release

- Update version in `tauri.conf.json`
- Create a new release with proper tag
- Github action will take over and upload the executable
