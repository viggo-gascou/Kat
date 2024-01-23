
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'kat' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'kat'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'kat' {
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('config', 'config', [CompletionResultType]::ParameterValue, 'Commands to help you configure kat')
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Get a problem from kattis')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialise the configuration files.')
            [CompletionResult]::new('open', 'open', [CompletionResultType]::ParameterValue, 'Open a problem in the browser')
            [CompletionResult]::new('submit', 'submit', [CompletionResultType]::ParameterValue, 'Submit a problem to kattis')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'Test a problem against its test case(s)')
            [CompletionResult]::new('watch', 'watch', [CompletionResultType]::ParameterValue, 'Watch a problem file for changes, and automatically test it when it changes')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'kat;config' {
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('locate', 'locate', [CompletionResultType]::ParameterValue, 'Locate the configuration files.')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set the location of where the configuration files are stored.')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'kat;config;locate' {
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'kat;config;set' {
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'kat;config;help' {
            [CompletionResult]::new('locate', 'locate', [CompletionResultType]::ParameterValue, 'Locate the configuration files.')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set the location of where the configuration files are stored.')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'kat;config;help;locate' {
            break
        }
        'kat;config;help;set' {
            break
        }
        'kat;config;help;help' {
            break
        }
        'kat;get' {
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'The path where you want to download the problem. 
If not specified, the problem will be downloaded to the current directory.')
            [CompletionResult]::new('--path', 'path', [CompletionResultType]::ParameterName, 'The path where you want to download the problem. 
If not specified, the problem will be downloaded to the current directory.')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'The programming language to setup the problem for.
If not specified, the language will be determined default language in the configuration file.')
            [CompletionResult]::new('--language', 'language', [CompletionResultType]::ParameterName, 'The programming language to setup the problem for.
If not specified, the language will be determined default language in the configuration file.')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'kat;init' {
            [CompletionResult]::new('-y', 'y', [CompletionResultType]::ParameterName, 'If set, you will not be prompted to confirm whether you want to overwrite existing config files.')
            [CompletionResult]::new('--yes', 'yes', [CompletionResultType]::ParameterName, 'If set, you will not be prompted to confirm whether you want to overwrite existing config files.')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'kat;open' {
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'kat;submit' {
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'The path of the solution file to submit. 
If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'The path of the solution file to submit. 
If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.')
            [CompletionResult]::new('--language', 'language', [CompletionResultType]::ParameterName, 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'If set, the the problem will be run against all of the local test cases before submitting.
If the problem fails any of the test cases, the submission will be aborted.')
            [CompletionResult]::new('--test-first', 'test-first', [CompletionResultType]::ParameterName, 'If set, the the problem will be run against all of the local test cases before submitting.
If the problem fails any of the test cases, the submission will be aborted.')
            [CompletionResult]::new('-y', 'y', [CompletionResultType]::ParameterName, 'If set, you will not be prompted to confirm the submission before it is sent to kattis.')
            [CompletionResult]::new('--yes', 'yes', [CompletionResultType]::ParameterName, 'If set, you will not be prompted to confirm the submission before it is sent to kattis.')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'If set, the submission will be opened in the browser after all tests have been run on the kattis server.')
            [CompletionResult]::new('--open', 'open', [CompletionResultType]::ParameterName, 'If set, the submission will be opened in the browser after all tests have been run on the kattis server.')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'kat;test' {
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'The path of the solution file to test. If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'The path of the solution file to test. If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'The id(s) of the test case(s) to test against. 
If not specified, all test cases will be tested, e.g. ''1'', ''1-3'', or ''1,3-5''.')
            [CompletionResult]::new('--test-cases', 'test-cases', [CompletionResultType]::ParameterName, 'The id(s) of the test case(s) to test against. 
If not specified, all test cases will be tested, e.g. ''1'', ''1-3'', or ''1,3-5''.')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.')
            [CompletionResult]::new('--language', 'language', [CompletionResultType]::ParameterName, 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'If set, try to submit the problem to kattis if all tests pass.')
            [CompletionResult]::new('--submit', 'submit', [CompletionResultType]::ParameterName, 'If set, try to submit the problem to kattis if all tests pass.')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'kat;watch' {
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'The path of the solution file to watch. 
If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'The path of the solution file to watch. 
If not specified, the first file with the same name as the problem in the problem folder will be used.
If multiple files with the correct extension are found, you will be prompted to choose which one to use.')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'The id(s) of the test case(s) to test against.
If not specified, all test cases will be tested, e.g. ''1'', ''1-3'', or ''1,3-5''.')
            [CompletionResult]::new('--test-cases', 'test-cases', [CompletionResultType]::ParameterName, 'The id(s) of the test case(s) to test against.
If not specified, all test cases will be tested, e.g. ''1'', ''1-3'', or ''1,3-5''.')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.')
            [CompletionResult]::new('--language', 'language', [CompletionResultType]::ParameterName, 'The programming language to test the problem against. 
This can be used to override the default language set in the configuration file.')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'kat;help' {
            [CompletionResult]::new('config', 'config', [CompletionResultType]::ParameterValue, 'Commands to help you configure kat')
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Get a problem from kattis')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialise the configuration files.')
            [CompletionResult]::new('open', 'open', [CompletionResultType]::ParameterValue, 'Open a problem in the browser')
            [CompletionResult]::new('submit', 'submit', [CompletionResultType]::ParameterValue, 'Submit a problem to kattis')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'Test a problem against its test case(s)')
            [CompletionResult]::new('watch', 'watch', [CompletionResultType]::ParameterValue, 'Watch a problem file for changes, and automatically test it when it changes')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'kat;help;config' {
            [CompletionResult]::new('locate', 'locate', [CompletionResultType]::ParameterValue, 'Locate the configuration files.')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set the location of where the configuration files are stored.')
            break
        }
        'kat;help;config;locate' {
            break
        }
        'kat;help;config;set' {
            break
        }
        'kat;help;get' {
            break
        }
        'kat;help;init' {
            break
        }
        'kat;help;open' {
            break
        }
        'kat;help;submit' {
            break
        }
        'kat;help;test' {
            break
        }
        'kat;help;watch' {
            break
        }
        'kat;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
