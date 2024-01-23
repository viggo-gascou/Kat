complete -c kat -n "__fish_use_subcommand" -s v -l verbose -d 'Increase logging verbosity'
complete -c kat -n "__fish_use_subcommand" -s q -l quiet -d 'Decrease logging verbosity'
complete -c kat -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c kat -n "__fish_use_subcommand" -s V -l version -d 'Print version'
complete -c kat -n "__fish_use_subcommand" -f -a "config" -d 'Commands to help you configure kat'
complete -c kat -n "__fish_use_subcommand" -f -a "get" -d 'Get a problem from kattis'
complete -c kat -n "__fish_use_subcommand" -f -a "init" -d 'Initialise the configuration files.'
complete -c kat -n "__fish_use_subcommand" -f -a "open" -d 'Open a problem in the browser'
complete -c kat -n "__fish_use_subcommand" -f -a "submit" -d 'Submit a problem to kattis'
complete -c kat -n "__fish_use_subcommand" -f -a "test" -d 'Test a problem against its test case(s)'
complete -c kat -n "__fish_use_subcommand" -f -a "watch" -d 'Watch a problem file for changes, and automatically test it when it changes'
complete -c kat -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c kat -n "__fish_seen_subcommand_from config; and not __fish_seen_subcommand_from locate; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from help" -s v -l verbose -d 'Increase logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from config; and not __fish_seen_subcommand_from locate; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from help" -s q -l quiet -d 'Decrease logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from config; and not __fish_seen_subcommand_from locate; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from help" -s h -l help -d 'Print help'
complete -c kat -n "__fish_seen_subcommand_from config; and not __fish_seen_subcommand_from locate; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from help" -f -a "locate" -d 'Locate the configuration files.'
complete -c kat -n "__fish_seen_subcommand_from config; and not __fish_seen_subcommand_from locate; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from help" -f -a "set" -d 'Set the location of where the configuration files are stored.'
complete -c kat -n "__fish_seen_subcommand_from config; and not __fish_seen_subcommand_from locate; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c kat -n "__fish_seen_subcommand_from config; and __fish_seen_subcommand_from locate" -s v -l verbose -d 'Increase logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from config; and __fish_seen_subcommand_from locate" -s q -l quiet -d 'Decrease logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from config; and __fish_seen_subcommand_from locate" -s h -l help -d 'Print help'
complete -c kat -n "__fish_seen_subcommand_from config; and __fish_seen_subcommand_from set" -s v -l verbose -d 'Increase logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from config; and __fish_seen_subcommand_from set" -s q -l quiet -d 'Decrease logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from config; and __fish_seen_subcommand_from set" -s h -l help -d 'Print help'
complete -c kat -n "__fish_seen_subcommand_from config; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from locate; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from help" -f -a "locate" -d 'Locate the configuration files.'
complete -c kat -n "__fish_seen_subcommand_from config; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from locate; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from help" -f -a "set" -d 'Set the location of where the configuration files are stored.'
complete -c kat -n "__fish_seen_subcommand_from config; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from locate; and not __fish_seen_subcommand_from set; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c kat -n "__fish_seen_subcommand_from get" -s p -l path -d 'The path where you want to download the problem. 
If not specified, the problem will be downloaded to the current directory.' -r -f -a "(__fish_complete_directories)"
complete -c kat -n "__fish_seen_subcommand_from get" -s l -l language -d 'The programming language to setup the problem for.
If not specified, the language will be determined default language in the configuration file.' -r
complete -c kat -n "__fish_seen_subcommand_from get" -s v -l verbose -d 'Increase logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from get" -s q -l quiet -d 'Decrease logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from get" -s h -l help -d 'Print help'
complete -c kat -n "__fish_seen_subcommand_from init" -s y -l yes -d 'If set, you will not be prompted to confirm whether you want to overwrite existing config files.'
complete -c kat -n "__fish_seen_subcommand_from init" -s v -l verbose -d 'Increase logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from init" -s q -l quiet -d 'Decrease logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from init" -s h -l help -d 'Print help'
complete -c kat -n "__fish_seen_subcommand_from open" -s v -l verbose -d 'Increase logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from open" -s q -l quiet -d 'Decrease logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from open" -s h -l help -d 'Print help'
complete -c kat -n "__fish_seen_subcommand_from submit" -s f -l file -d 'The path of the solution file to submit. 
If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.' -r -F
complete -c kat -n "__fish_seen_subcommand_from submit" -s l -l language -d 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.' -r
complete -c kat -n "__fish_seen_subcommand_from submit" -s t -l test-first -d 'If set, the the problem will be run against all of the local test cases before submitting.
If the problem fails any of the test cases, the submission will be aborted.'
complete -c kat -n "__fish_seen_subcommand_from submit" -s y -l yes -d 'If set, you will not be prompted to confirm the submission before it is sent to kattis.'
complete -c kat -n "__fish_seen_subcommand_from submit" -s o -l open -d 'If set, the submission will be opened in the browser after all tests have been run on the kattis server.'
complete -c kat -n "__fish_seen_subcommand_from submit" -s v -l verbose -d 'Increase logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from submit" -s q -l quiet -d 'Decrease logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from submit" -s h -l help -d 'Print help'
complete -c kat -n "__fish_seen_subcommand_from test" -s f -l file -d 'The path of the solution file to test. If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.' -r -F
complete -c kat -n "__fish_seen_subcommand_from test" -s t -l test-cases -d 'The id(s) of the test case(s) to test against. 
If not specified, all test cases will be tested, e.g. \'1\', \'1-3\', or \'1,3-5\'.' -r
complete -c kat -n "__fish_seen_subcommand_from test" -s l -l language -d 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.' -r
complete -c kat -n "__fish_seen_subcommand_from test" -s s -l submit -d 'If set, try to submit the problem to kattis if all tests pass.'
complete -c kat -n "__fish_seen_subcommand_from test" -s v -l verbose -d 'Increase logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from test" -s q -l quiet -d 'Decrease logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from test" -s h -l help -d 'Print help'
complete -c kat -n "__fish_seen_subcommand_from watch" -s f -l file -d 'The path of the solution file to watch. 
If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.' -r -F
complete -c kat -n "__fish_seen_subcommand_from watch" -s t -l test-cases -d 'The id(s) of the test case(s) to test against.
If not specified, all test cases will be tested, e.g. \'1\', \'1-3\', or \'1,3-5\'.' -r
complete -c kat -n "__fish_seen_subcommand_from watch" -s l -l language -d 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.' -r
complete -c kat -n "__fish_seen_subcommand_from watch" -s v -l verbose -d 'Increase logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from watch" -s q -l quiet -d 'Decrease logging verbosity'
complete -c kat -n "__fish_seen_subcommand_from watch" -s h -l help -d 'Print help'
complete -c kat -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from config; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from open; and not __fish_seen_subcommand_from submit; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from help" -f -a "config" -d 'Commands to help you configure kat'
complete -c kat -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from config; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from open; and not __fish_seen_subcommand_from submit; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from help" -f -a "get" -d 'Get a problem from kattis'
complete -c kat -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from config; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from open; and not __fish_seen_subcommand_from submit; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from help" -f -a "init" -d 'Initialise the configuration files.'
complete -c kat -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from config; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from open; and not __fish_seen_subcommand_from submit; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from help" -f -a "open" -d 'Open a problem in the browser'
complete -c kat -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from config; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from open; and not __fish_seen_subcommand_from submit; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from help" -f -a "submit" -d 'Submit a problem to kattis'
complete -c kat -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from config; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from open; and not __fish_seen_subcommand_from submit; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from help" -f -a "test" -d 'Test a problem against its test case(s)'
complete -c kat -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from config; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from open; and not __fish_seen_subcommand_from submit; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from help" -f -a "watch" -d 'Watch a problem file for changes, and automatically test it when it changes'
complete -c kat -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from config; and not __fish_seen_subcommand_from get; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from open; and not __fish_seen_subcommand_from submit; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c kat -n "__fish_seen_subcommand_from help; and __fish_seen_subcommand_from config; and not __fish_seen_subcommand_from locate; and not __fish_seen_subcommand_from set" -f -a "locate" -d 'Locate the configuration files.'
complete -c kat -n "__fish_seen_subcommand_from help; and __fish_seen_subcommand_from config; and not __fish_seen_subcommand_from locate; and not __fish_seen_subcommand_from set" -f -a "set" -d 'Set the location of where the configuration files are stored.'
