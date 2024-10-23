"""Console script for quant_de."""
import click
from rich.console import Console

console = Console()


@click.group()
def cli():
    """Quant DE Command Line Interface."""
    pass


@cli.command()
def main():
    """Main command for quant_de."""
    console.print("Replace this message by putting your code into "
                  "quant_de.cli.main", style="bold green")
    console.print("See Click documentation at https://click.palletsprojects.com/", style="italic blue")


if __name__ == "__main__":
    cli()
