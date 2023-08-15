# CRust - Project Manager written in Rust with Tauri

CRust is a project manager written in Rust and built using Tauri. It allows you to efficiently manage your projects, workspaces, and project types. With a user-friendly graphical interface, CRust streamlines your project organization and development workflow.

## Features

- Add workspaces with project directories matching project type configurations.
- Quicksearch for projects to easily locate and access them.
- Run selected projects with Visual Studio Code (VSCode), assuming it's installed.
- Manage workspaces by reindexing or deleting them.
- Add, modify, or delete project types with customizable coloring.
- Planned features: custom run configurations, enhanced workspace indexing, specific project language support (C++, Java, etc.), and more.

## Installation

1. Download the latest release version of CRust.
2. Ensure you have Visual Studio Code (VSCode) installed.
3. Simply run the executable (or platform-specific application) to launch CRust.

## Contributing

Contributions to CRust are highly welcome! Whether you're interested in contributing code, suggesting new features, discussing improvements, or participating in project discussions, your input is valuable.

- Fork this repository and create a new branch for your contributions.
- Submit pull requests to propose changes or additions.
- Join discussions in issues to share your ideas and feedback.
- Feel free to restructure or enhance the project as needed.

## License

CRust is licensed under the MPL v2 License. See [LICENSE](LICENSE) for more details.

## Contact

For questions, suggestions, or feedback, feel free to reach out:

- GitHub Issues: [Project Issues](https://github.com/CubesCoders/crust-gui/issues)
- Discord: @cubecoder

## The config.json

In the root directory you can find or create a config.json file where the project types are stored.
Because there are no pre-defined project types the config will be empty and thus no projects from workspaces will be added.
Here is a config.json example that you can use:
```json
{
  "project_types": [
    {
      "id": "0",
      "name": "rust [bin]",
      "needed_files": [
        "Cargo.toml",
        "src/main.rs"
      ],
      "color": "#f74b00"
    },
    {
      "id": "1",
      "name": "tauri",
      "needed_files": [
        "src-tauri/tauri.conf.json",
        "src-tauri/src/main.rs",
        "src-tauri/Cargo.toml"
      ],
      "color": "#23a8b7"
    },
    {
      "id": "2",
      "name": "godot",
      "needed_files": [
        "project.godot"
      ],
      "color": "#4489bb"
    },
    {
      "id": "3",
      "name": "rust [lib]",
      "needed_files": [
        "Cargo.toml",
        "src/lib.rs"
      ],
      "color": "#f74b00"
    },
    {
      "id": "4",
      "name": "Web",
      "needed_files": [
        "index.html"
      ],
      "color": "#fff"
    }
  ]
}
```

## Galerie

This is how it will look like on startup if you have (active) workspaces
![startup](https://github.com/CubesCoders/crust-gui/assets/34133543/460cc5b4-d56b-4cda-b0d4-6378f63522a3)

You can just start to type if the window has focus. The first entry will be selected with the arrow keys you can navigate up and down. When the enter key is pressed the selected project will start in vscode
![search](https://github.com/CubesCoders/crust-gui/assets/34133543/a84fb9ad-e6d8-4429-bb2d-8f01757c2be0)

Here are some settings that will be more or less self-explanatory (Note: There are no predefined project types add some to successfully add projects from workspaces)

<img src="https://github.com/CubesCoders/crust-gui/assets/34133543/7dfc557a-8296-4d59-be32-430d4cb5106e" width="500px" />
<img src="https://github.com/CubesCoders/crust-gui/assets/34133543/8a6fc35b-0e5a-43bc-8d2b-a593e0dcf5a0" width="500px" />

