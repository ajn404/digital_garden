use color_eyre::eyre::Result;
use structopt::StructOpt;

#[derive(StructOpt,Debug)]
#[structopt(name="garden")]


/// A CLI for the growing and curation of a digital garden
/// 
/// Visit https://www.rustadventure.rs/garden for more

struct Opt{
    #[structopt(subcommand)]
    cmd:Command,
}

#[derive(StructOpt,Debug)]
enum Command{

    /// Write somthing in your garden 
    /// 
    /// This command will open your $EDITOR,wait for you 
    /// to write somthing , and then save the file to your garden

    Write{
        #[structopt(short,long)]
        title:Option<String>
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    dbg!(opt);
    todo!();
}



