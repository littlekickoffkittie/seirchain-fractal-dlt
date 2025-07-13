@if "%DEBUG%" == "" @echo off
@rem ##########################################################################
@rem
@rem  Gradle startup script for Windows
@rem
@rem ##########################################################################

@rem Set local scope for the variables with windows NT shell
if "%OS%"=="Windows_NT" setlocal

set DIRNAME=%~dp0
if "%DIRNAME%" == "" set DIRNAME=.
set APP_BASE_NAME=%~n0
set APP_HOME=%DIRNAME%

@rem Add default JVM options here. You can also use JAVA_OPTS and GRADLE_OPTS to pass any JVM options to Gradle and Java processes.
set DEFAULT_JVM_OPTS=

@rem Find java.exe
if defined JAVA_HOME goto findJavaFromJavaHome

set JAVA_EXE=java.exe
%JAVA_EXE% -version >NUL 2>&1
if "%ERRORLEVEL%" == "0" goto init

echo.
echo ERROR: JAVA_HOME is not set and no 'java' command could be found in your PATH.
echo.
echo Please set the JAVA_HOME variable in your environment to match the
echo location of your Java installation.

goto fail

:findJavaFromJavaHome
set JAVA_HOME=%JAVA_HOME:"=%
set JAVA_EXE=%JAVA_HOME%/bin/java.exe

if exist "%JAVA_EXE%" goto init

echo.
echo ERROR: JAVA_HOME is set to an invalid directory: %JAVA_HOME%
echo.
echo Please set the JAVA_HOME variable in your environment to match the
echo location of your Java installation.

goto fail

:init
@rem Get command-line arguments, handling Windowz variants

if not "%OS%" == "Windows_NT" goto oldWindows
:checkVer
if not "%~1" == "" (
    if /i "%~1" == "/v" (
        set _SAVED_ARGS=%*
        set _SKIP=2
        if /i "%~2" == "/?" (
            set _SKIP=3
        )
        if /i "%~2" == "-?" (
            set _SKIP=3
        )
        if /i "%~2" == "--help" (
            set _SKIP=3
        )
    )
)

if not "%OS%" == "Windows_NT" goto oldWindows
if "%~1"=="" goto main
set "CMDLINEREM=%1"
set "ARGS="
:strip
if not "%~1"=="" (
    set "ARGS=%ARGS% %1"
    shift
    goto strip
)
goto main

:oldWindows
if "%OS%"=="Windows_NT" setlocal
set CMD_LINE_ARGS=
:oldWinLoop
if "%1"=="" goto main
set CMD_LINE_ARGS=%CMD_LINE_ARGS% %1
shift
goto oldWinLoop

:main
@rem Set this to point to your gradle installation
if not defined GRADLE_HOME set GRADLE_HOME=%APP_HOME%

@rem Add the launcher to the CLASSPATH
set CLASSPATH=%APP_HOME%\gradle\wrapper\gradle-wrapper.jar

@rem Execute Gradle
"%JAVA_EXE%" %DEFAULT_JVM_OPTS% %JAVA_OPTS% %GRADLE_OPTS% "-Dorg.gradle.appname=%APP_BASE_NAME%" -classpath "%CLASSPATH%" org.gradle.wrapper.GradleWrapperMain %CMD_LINE_ARGS% %ARGS%

:end
@rem End local scope for the variables with windows NT shell
if "%ERRORLEVEL%"=="0" goto mainEnd

:fail
rem Set variable required for error recovery
set EXIT_CODE=%ERRORLEVEL%
if not "%EXIT_CODE%"=="" (
  if /i "%EXIT_CODE%" NEQ "0" (
    if not defined NO_EXIT_ON_FAILURE (
      exit /b %EXIT_CODE%
    )
  )
)

:mainEnd
if "%OS%"=="Windows_NT" endlocal

:omega
