
use builtin;
use str;

set edit:completion:arg-completer[kat] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'kat'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'kat'= {
            cand -v 'Increase logging verbosity'
            cand --verbose 'Increase logging verbosity'
            cand -q 'Decrease logging verbosity'
            cand --quiet 'Decrease logging verbosity'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand config 'Commands to help you configure kat'
            cand get 'Get a problem from kattis'
            cand init 'Initialise the configuration files'
            cand open 'Open a problem in the browser'
            cand submit 'Submit a problem to kattis'
            cand test 'Test a problem against its test case(s)'
            cand watch 'Watch a problem file for changes, and automatically test it when it changes'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'kat;config'= {
            cand -v 'Increase logging verbosity'
            cand --verbose 'Increase logging verbosity'
            cand -q 'Decrease logging verbosity'
            cand --quiet 'Decrease logging verbosity'
            cand -h 'Print help'
            cand --help 'Print help'
            cand locate 'Locate the configuration files.'
            cand set 'Set the location of where the configuration files are stored.'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'kat;config;locate'= {
            cand -v 'Increase logging verbosity'
            cand --verbose 'Increase logging verbosity'
            cand -q 'Decrease logging verbosity'
            cand --quiet 'Decrease logging verbosity'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'kat;config;set'= {
            cand -v 'Increase logging verbosity'
            cand --verbose 'Increase logging verbosity'
            cand -q 'Decrease logging verbosity'
            cand --quiet 'Decrease logging verbosity'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'kat;config;help'= {
            cand locate 'Locate the configuration files.'
            cand set 'Set the location of where the configuration files are stored.'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'kat;config;help;locate'= {
        }
        &'kat;config;help;set'= {
        }
        &'kat;config;help;help'= {
        }
        &'kat;get'= {
            cand -p 'The path where you want to download the problem. 
If not specified, the problem will be downloaded to the current directory.'
            cand --path 'The path where you want to download the problem. 
If not specified, the problem will be downloaded to the current directory.'
            cand -l 'The programming language to setup the problem for.
If not specified, the language will be determined default language in the configuration file.'
            cand --language 'The programming language to setup the problem for.
If not specified, the language will be determined default language in the configuration file.'
            cand -v 'Increase logging verbosity'
            cand --verbose 'Increase logging verbosity'
            cand -q 'Decrease logging verbosity'
            cand --quiet 'Decrease logging verbosity'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'kat;init'= {
            cand -y 'If set, you will not be prompted to confirm whether you want to overwrite existing config files.'
            cand --yes 'If set, you will not be prompted to confirm whether you want to overwrite existing config files.'
            cand -v 'Increase logging verbosity'
            cand --verbose 'Increase logging verbosity'
            cand -q 'Decrease logging verbosity'
            cand --quiet 'Decrease logging verbosity'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'kat;open'= {
            cand -v 'Increase logging verbosity'
            cand --verbose 'Increase logging verbosity'
            cand -q 'Decrease logging verbosity'
            cand --quiet 'Decrease logging verbosity'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'kat;submit'= {
            cand -f 'The path of the solution file to submit. 
If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.'
            cand --file 'The path of the solution file to submit. 
If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.'
            cand -l 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.'
            cand --language 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.'
            cand -t 'If set, the the problem will be run against all of the local test cases before submitting.
If the problem fails any of the test cases, the submission will be aborted.'
            cand --test-first 'If set, the the problem will be run against all of the local test cases before submitting.
If the problem fails any of the test cases, the submission will be aborted.'
            cand -y 'If set, you will not be prompted to confirm the submission before it is sent to kattis.'
            cand --yes 'If set, you will not be prompted to confirm the submission before it is sent to kattis.'
            cand -o 'If set, the submission will be opened in the browser after all tests have been run on the kattis server.'
            cand --open 'If set, the submission will be opened in the browser after all tests have been run on the kattis server.'
            cand -v 'Increase logging verbosity'
            cand --verbose 'Increase logging verbosity'
            cand -q 'Decrease logging verbosity'
            cand --quiet 'Decrease logging verbosity'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'kat;test'= {
            cand -f 'The path of the solution file to test. If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.'
            cand --file 'The path of the solution file to test. If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.'
            cand -t 'The id(s) of the test case(s) to test against. 
If not specified, all test cases will be tested, e.g. ''1'', ''1-3'', or ''1,3-5''.'
            cand --test-cases 'The id(s) of the test case(s) to test against. 
If not specified, all test cases will be tested, e.g. ''1'', ''1-3'', or ''1,3-5''.'
            cand -l 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.'
            cand --language 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.'
            cand -s 'If set, try to submit the problem to kattis if all tests pass.'
            cand --submit 'If set, try to submit the problem to kattis if all tests pass.'
            cand -v 'Increase logging verbosity'
            cand --verbose 'Increase logging verbosity'
            cand -q 'Decrease logging verbosity'
            cand --quiet 'Decrease logging verbosity'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'kat;watch'= {
            cand -f 'The path of the solution file to watch. 
If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.'
            cand --file 'The path of the solution file to watch. 
If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.'
            cand -t 'The id(s) of the test case(s) to test against.
If not specified, all test cases will be tested, e.g. ''1'', ''1-3'', or ''1,3-5''.'
            cand --test-cases 'The id(s) of the test case(s) to test against.
If not specified, all test cases will be tested, e.g. ''1'', ''1-3'', or ''1,3-5''.'
            cand -l 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.'
            cand --language 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.'
            cand -v 'Increase logging verbosity'
            cand --verbose 'Increase logging verbosity'
            cand -q 'Decrease logging verbosity'
            cand --quiet 'Decrease logging verbosity'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'kat;help'= {
            cand config 'Commands to help you configure kat'
            cand get 'Get a problem from kattis'
            cand init 'Initialise the configuration files'
            cand open 'Open a problem in the browser'
            cand submit 'Submit a problem to kattis'
            cand test 'Test a problem against its test case(s)'
            cand watch 'Watch a problem file for changes, and automatically test it when it changes'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'kat;help;config'= {
            cand locate 'Locate the configuration files.'
            cand set 'Set the location of where the configuration files are stored.'
        }
        &'kat;help;config;locate'= {
        }
        &'kat;help;config;set'= {
        }
        &'kat;help;get'= {
        }
        &'kat;help;init'= {
        }
        &'kat;help;open'= {
        }
        &'kat;help;submit'= {
        }
        &'kat;help;test'= {
        }
        &'kat;help;watch'= {
        }
        &'kat;help;help'= {
        }
    ]
    $completions[$command]
}
