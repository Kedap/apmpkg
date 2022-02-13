function __fish_using_command
    set cmd (commandline -opc)
    if [ (count $cmd) -eq (count $argv) ]
        for i in (seq (count $argv))
            if [ $cmd[$i] != $argv[$i] ]
                return 1
            end
        end
        return 0
    end
    return 1
end

complete -c apmpkg -n "__fish_using_command apmpkg" -s d -l instalar_dependencia -d "Instala la dependencia especificada"
complete -c apmpkg -n "__fish_using_command apmpkg" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg" -s V -l version -d "Prints version information"
complete -c apmpkg -n "__fish_using_command apmpkg" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg" -s V -l version -d "Prints version information"
complete -c apmpkg -n "__fish_using_command apmpkg" -f -a "instalar"
complete -c apmpkg -n "__fish_using_command apmpkg" -f -a "remover"
complete -c apmpkg -n "__fish_using_command apmpkg" -f -a "crear"
complete -c apmpkg -n "__fish_using_command apmpkg" -f -a "construir"
complete -c apmpkg -n "__fish_using_command apmpkg" -f -a "help"
complete -c apmpkg -n "__fish_using_command apmpkg" -f -a "help"
complete -c apmpkg -n "__fish_using_command apmpkg instalar" -s u -l url -d "Direccion URL del archivo adi o abc"
complete -c apmpkg -n "__fish_using_command apmpkg instalar" -s c -l confirmar -d "No se interactua con el usuario en la confirmacion"
complete -c apmpkg -n "__fish_using_command apmpkg instalar" -s b -l binario -d "Creacion de un binario apartir de un adi o abc"
complete -c apmpkg -n "__fish_using_command apmpkg instalar" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg instalar" -s V -l version -d "Prints version information"
complete -c apmpkg -n "__fish_using_command apmpkg instalar" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg instalar" -s V -l version -d "Prints version information"
complete -c apmpkg -n "__fish_using_command apmpkg remover" -s c -l confirmar -d "No se interactua con el usuario en la confirmacion"
complete -c apmpkg -n "__fish_using_command apmpkg remover" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg remover" -s V -l version -d "Prints version information"
complete -c apmpkg -n "__fish_using_command apmpkg remover" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg remover" -s V -l version -d "Prints version information"
complete -c apmpkg -n "__fish_using_command apmpkg crear" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg crear" -s V -l version -d "Prints version information"
complete -c apmpkg -n "__fish_using_command apmpkg crear" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg crear" -s V -l version -d "Prints version information"
complete -c apmpkg -n "__fish_using_command apmpkg construir" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg construir" -s V -l version -d "Prints version information"
complete -c apmpkg -n "__fish_using_command apmpkg construir" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg construir" -s V -l version -d "Prints version information"
complete -c apmpkg -n "__fish_using_command apmpkg help" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg help" -s V -l version -d "Prints version information"
complete -c apmpkg -n "__fish_using_command apmpkg help" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg help" -s V -l version -d "Prints version information"
complete -c apmpkg -n "__fish_using_command apmpkg help" -s h -l help -d "Prints help information"
complete -c apmpkg -n "__fish_using_command apmpkg help" -s V -l version -d "Prints version information"
