Describe 'registry config test tests' {
    It 'Can test a registry key <test>' -TestCases @(
        @{ test = 'exists and present'; ensure = 'Present'; key = 'CurrentVersion' }
        @{ test = 'does not exist and absent'; ensure = 'Absent'; key = 'DoesNotExist' }
    ){
        param($ensure, $key)
        $json = @"
        {
          "keyPath": "HKLM\\Software\\Microsoft\\Windows NT\\$key",
          "_ensure": "$ensure"
        }
"@
        $out = $json | registry config test
        $LASTEXITCODE | Should -Be 0
        $result = $out | ConvertFrom-Json
        $result.keyPath | Should -BeNullOrEmpty
        ($result.psobject.properties | Measure-Object).Count | Should -Be 1
    }

    It 'Can report failure if a registry key <test>' -TestCases @(
        @{ test = 'exists and absent'; ensure = 'Absent'; key = 'CurrentVersion'; expectedKey = 'HKLM\Software\Microsoft\Windows NT\CurrentVersion' }
        @{ test = 'does not exist and present'; ensure = 'Present'; key = 'DoesNotExist'; expectedKey = '' }
    ){
        param($ensure, $key, $expectedKey)
        $json = @"
        {
          "keyPath": "HKLM\\Software\\Microsoft\\Windows NT\\$key",
          "_ensure": "$ensure"
        }
"@
        $out = $json | registry config test
        $LASTEXITCODE | Should -Be 4
        $result = $out | ConvertFrom-Json
        $result.keyPath | Should -BeExactly $expectedKey
        ($result.psobject.properties | Measure-Object).Count | Should -Be 1
    }

    It 'Can test a registry value exists' {
        $json = @"
        {
          "keyPath": "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion",
          "valueName": "ProgramFilesPath",
          "_ensure": "Present"
        }
"@
        $out = $json | registry config test
        $LASTEXITCODE | Should -Be 0
        $result = $out | ConvertFrom-Json
        $result.keyPath | Should -BeExactly 'HKLM\Software\Microsoft\Windows\CurrentVersion'
        $result.valueName | Should -BeExactly 'ProgramFilesPath'
        $result.valueData.ExpandString | Should -BeExactly '%ProgramFiles%'
        ($result.psobject.properties | Measure-Object).Count | Should -Be 3
    }
}