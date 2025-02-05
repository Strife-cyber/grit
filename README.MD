# **Grit - A Lightweight Version Control System**

**Grit** is a lightweight, Git-like version control system designed for tracking changes in files and managing commits. It allows you to initialize repositories, add files, and commit changes while maintaining a simple and efficient structure.

------

## **🚀 Features**

✅ **Initialize a repository** – Create a `.grit` directory to start tracking files.
✅ **Add files** – Stage files for commit.
✅ **Commit changes** – Save file changes with a commit message.
✅ **Track file versions** – Maintain a history of modifications.
✅ **Branching** – Support for multiple branches.
✅ **Logging Commits** – Display commit history in a readable format.

### **🔜 Future Enhancements**

🔹 **Merging** – Ability to merge commit histories.
🔹 **Commit Diffing** – Compare two commits to see changes.
🔹 **Undoing Commits** – Soft/hard resets to previous commits.

------

## **📦 Installation**

Ensure you have **Rust** installed. Clone the repository and build the project:

```
shCopyEdit
git clone https://github.com/Strife-Cyber/grit.git
cd grit
cargo build --release
```

This will generate an executable inside `target/release/`.

------

## **🛠 Usage**

Once built, you can run **Grit** using the `grit` command:

### **🔹 Initialize a repository**

```
sh


CopyEdit
grit init
```

Creates a `.grit` directory to start tracking files.

### **🔹 Add files**

```
sh


CopyEdit
grit add <file>
```

Stages a file for commit. Use `"."` to add all files.

### **🔹 Commit changes**

```
sh


CopyEdit
grit commit -m "Your commit message"
```

Commits the staged files with a message.

------

## **👨‍💻 Development**

To run tests:

```
sh


CopyEdit
cargo test
```

To run the program:

```
sh


CopyEdit
cargo run -- <command> [args]
```

Example:

```
sh


CopyEdit
cargo run -- init
```

------

## **🤝 Contributing**

Feel free to fork this project and submit pull requests for improvements! 🚀

------

## **📜 License**

This project is open-source and available under the **MIT License**.