"""Console script for quant_de."""
import quant_de
import typer
from rich.console import Console

app = typer.Typer()
console = Console()


@app.command()
def main():
    """Console script for quant_de."""
    console.print("Replace this message by putting your code into "
               "quant_de.cli.main")
    console.print("See Typer documentation at https://typer.tiangolo.com/")
    


if __name__ == "__main__":
    app()
