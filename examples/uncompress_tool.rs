// Copyright (C) 2019-2021 O.S. Systems Software LTDA
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use argh::FromArgs;
use compress_tools::*;
use std::path::Path;

#[derive(FromArgs, PartialEq, Eq, Debug)]
/// Top-level command.
struct TopLevel {
    #[argh(subcommand)]
    nested: CmdLine,
}

#[derive(FromArgs, PartialEq, Eq, Debug)]
#[argh(subcommand)]
enum CmdLine {
    ListArchiveFiles(SubCommandListArchiveFiles),
    UncompressData(SubCommandUncompressData),
    UncompressArchiveFile(SubCommandUncompressArchiveFile),
    UncompressArchive(SubCommandUncompressArchive),
}

#[derive(FromArgs, PartialEq, Eq, Debug)]
/// Uncompress data subcommand.
#[argh(subcommand, name = "uncompress-data")]
struct SubCommandUncompressData {
    /// source path
    #[argh(positional)]
    source_path: String,

    /// target path
    #[argh(positional)]
    target_path: String,
}

#[derive(FromArgs, PartialEq, Eq, Debug)]
/// Uncompress archive file subcommand.
#[argh(subcommand, name = "uncompress-archive-file")]
struct SubCommandUncompressArchiveFile {
    /// source path
    #[argh(positional)]
    source_path: String,

    /// target path
    #[argh(positional)]
    target_path: String,

    /// target file
    #[argh(positional)]
    target_file: String,
}

#[derive(FromArgs, PartialEq, Eq, Debug)]
/// Uncompress archive subcommand.
#[argh(subcommand, name = "uncompress-archive")]
struct SubCommandUncompressArchive {
    /// source path
    #[argh(positional)]
    source_path: String,

    /// target path
    #[argh(positional)]
    target_path: String,

    /// whether or not to preserver ownership
    #[argh(positional)]
    preserve_ownership: bool,
}

#[derive(FromArgs, PartialEq, Eq, Debug)]
/// List archive files subcommand.
#[argh(subcommand, name = "list-archive-files")]
struct SubCommandListArchiveFiles {
    /// source path
    #[argh(positional)]
    source_path: String,
}

fn main() -> compress_tools::Result<()> {
    let cmd: TopLevel = argh::from_env();

    match cmd.nested {
        CmdLine::ListArchiveFiles(input) => {
            let filename = std::fs::File::open(input.source_path)?;

            let file_list = list_archive_files(filename)?;
            println!("{:#?}", file_list);
        }

        CmdLine::UncompressData(input) => {
            let mut source = std::fs::File::open(input.source_path)?;
            let mut target = std::fs::File::create(input.target_path)?;

            uncompress_data(&mut source, &mut target)?;
        }
        CmdLine::UncompressArchiveFile(input) => {
            let mut source = std::fs::File::open(input.source_path)?;
            let mut target = std::fs::File::create(input.target_path)?;

            uncompress_archive_file(&mut source, &mut target, &input.target_file)?;
        }
        CmdLine::UncompressArchive(input) => {
            let mut source = std::fs::File::open(input.source_path)?;

            uncompress_archive(
                &mut source,
                Path::new(&input.target_path),
                if input.preserve_ownership {
                    Ownership::Preserve
                } else {
                    Ownership::Ignore
                },
            )?;
        }
    }

    Ok(())
}
