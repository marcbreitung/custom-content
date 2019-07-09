# Custom Content

## WIP
Generate TYPO3 custom content elements via command line.

### Generate a new extension

With long arguments

```bash
custom-content extension --key <key> --title <title> --description <description>
```

With short arguments

```bash
custom-content extension -k <key> -t <title> -d <description>
```

Example

```bash
custom-content extension -k new_extension -t Title -d Description
```

### Add new content element

With long arguments

```bash
custom-content element --extension <extension> --key <key> --icon <icon> 
```

With short arguments

```bash
custom-content element -e <extension> -k <key> -i <icon> 
```

Example

```bash
custom-content element -e content_elements -k text -i text
```