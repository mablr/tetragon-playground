# Tetragon Playground

A demonstration environment for Cilium Tetragon, the eBPF-based security observability tool.

## Contents

- **tetragon/**: Contains TracingPolicy YAML files for monitoring file access operations
- **file_io/**: Rust project with file I/O examples to demonstrate Tetragon's monitoring capabilities
- **test/**: Test files with different permissions (.ro, .wo, .protected) for demonstrating file access monitoring

## Usage

This repository serves as a playground to explore Tetragon's capabilities for monitoring and enforcing security policies on file operations. 