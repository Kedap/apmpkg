use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(
    name = "ApmPKG",
    version = "v1.5.2",
    author = "kedap. <kedap.dev@protonmail.com>",
    about = "Un administrador de paquetes universal para linux como modelo PKGBUILD"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub comandos: Comandos,
    #[clap(
        help = "Instala la dependencia especificada",
        long = "instalar_dependencia",
        short = 'd',
        takes_value = true,
        required = false,
        default_value = ""
    )]
    pub dependencia: String,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Comandos {
    #[clap(about = "Instala/Actualiza un paquete")]
    Instalar {
        #[clap(
            help = "Nombre y/o ruta del archivo adi, abi o abc",
            takes_value = true,
            default_value = ""
        )]
        paquete: String,

        #[clap(
            help = "No se interactua con el usuario en la instalacion",
            long = "confirmar",
            short = 'c'
        )]
        confirmar: bool,

        #[clap(
            help = "Direccion URL del archivo adi o abc",
            long = "url",
            short = 'u',
            takes_value = true,
            required = false,
            default_value = ""
        )]
        url: String,

        #[clap(
            help = "Creacion de un binario despues de realizar la instalacion",
            long = "binario",
            short = 'b'
        )]
        binario: bool,
    },

    #[clap(about = "Desinstala un paquete")]
    Remover {
        #[clap(
            help = "Nombre del paquete a desinstalar",
            takes_value = true,
            required = true
        )]
        paquete: String,

        #[clap(
            help = "No se interactua con el usuario en la confirmacion",
            long = "confirmar",
            short = 'c'
        )]
        confirmar: bool,
    },

    #[clap(about = "Crea un prototipo para archivos adi o abc")]
    Crear {
        #[clap(
            help = "Tipo de paquete: adi | abc",
            takes_value = true,
            required = true
        )]
        tipo: String,

        #[clap(help = "Nombre del paquete", takes_value = true, required = true)]
        paquete: String,
    },

    #[clap(about = "Crea un binario apartir de un archivo adi o abc")]
    Construir {
        #[clap(
            help = "Ruta del archivo adi o abc a construir el binario",
            takes_value = true,
            required = true
        )]
        paquete: String,
    },
}
