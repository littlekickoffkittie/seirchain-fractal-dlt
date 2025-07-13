#!/usr/bin/env sh

#
# Copyright 2015 the original author or authors.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

# Add default JVM options here. You can also use JAVA_OPTS and GRADLE_OPTS to pass any JVM options to Gradle and Java processes.
DEFAULT_JVM_OPTS=""

APP_NAME="Gradle"
APP_BASE_NAME=`basename "$0"`

# Use the maximum available, or set MAX_FD != -1 to use that value.
MAX_FD="maximum"

# OS specific support.  $var _must_ be set to either true or false.
cygwin=false
msys=false
darwin=false
nonstop=false
case "`uname`" in
  CYGWIN* )
    cygwin=true
    ;;
  Darwin* )
    darwin=true
    ;;
  MINGW* )
    msys=true
    ;;
  NONSTOP* )
    nonstop=true
    ;;
esac

# Attempt to set APP_HOME
# Resolve links: $0 may be a link
PRG="$0"
# Need this for relative symlinks.
while [ -h "$PRG" ] ; do
    ls=`ls -ld "$PRG"`
    link=`expr "$ls" : '.*-> \(.*\)$'`
    if expr "$link" : '/.*' > /dev/null; then
        PRG="$link"
    else
        PRG=`dirname "$PRG"`"/$link"
    fi
done
SAVED="`pwd`"
cd "`dirname \"$PRG\"`/" >/dev/null
APP_HOME="`pwd -P`"
cd "$SAVED" >/dev/null

# Add a second backslash to variables coming from the properties file,
# so that they don't get immediately expanded by the shell.
# Note: if you are using variable expansion in the properties file,
# you must wrap it in double quotes, but not in single quotes.
resolve_props () {
    sed "s/\\\\/\\\\\\\\/g"
}

# Read properties file, if it exists.
if [ -f "$APP_HOME/gradle/wrapper/gradle-wrapper.properties" ]; then
    while IFS= read -r line; do
        eval "$line"
    done < "$APP_HOME/gradle/wrapper/gradle-wrapper.properties"
fi

# Set default GRADLE_USER_HOME if not set
if [ -z "$GRADLE_USER_HOME" ]; then
    GRADLE_USER_HOME="$HOME/.gradle"
fi

# Set distributionUrl if not set
if [ -z "$distributionUrl" ]; then
    echo "distributionUrl not set in gradle-wrapper.properties" 1>&2
    exit 1
fi

# Determine the name of the distribution directory
distributionUrl_basename=`basename $distributionUrl`
distribution_name=`echo $distributionUrl_basename | sed 's/\.zip$//'`
distribution_dir="$GRADLE_USER_HOME/wrapper/dists/$distribution_name"

# Determine the name of the distribution zip file
distribution_zip_name=`basename $distributionUrl`
distribution_zip_file="$distribution_dir/$distribution_zip_name"

# Determine the name of the gradle-launcher jar file
gradle_launcher_jar="$APP_HOME/gradle/wrapper/gradle-wrapper.jar"

# For Cygwin, switch paths to Windows format before running java
if $cygwin ; then
    APP_HOME=`cygpath --path --windows "$APP_HOME"`
    GRADLE_HOME=`cygpath --path --windows "$GRADLE_HOME"`
    GRADLE_USER_HOME=`cygpath --path --windows "$GRADLE_USER_HOME"`
    CLASSPATH=`cygpath --path --windows "$CLASSPATH"`
fi

# For MinGW, ensure paths are in UNIX format before anything else
if $msys ; then
    APP_HOME=`( cd "$APP_HOME" && pwd )`
    GRADLE_HOME=`( cd "$GRADLE_HOME" && pwd )`
    GRADLE_USER_HOME=`( cd "$GRADLE_USER_HOME" && pwd )`
    # Build the classpath from an ordered list of jars
    CLASSPATH=""
    for dir in `find "$GRADLE_HOME/lib" -type d | sort`; do
        for jar in `find "$dir" -type f -name "*.jar" | sort`; do
            CLASSPATH="$CLASSPATH:$jar"
        done
    done
fi

# Escape parenthesis for Ant paths
CLASSPATH=`echo $CLASSPATH | sed 's/(/\\\\(/g'`
CLASSPATH=`echo $CLASSPATH | sed 's/)/\\\\)/g'`

# No array support, use a simple string.
JVM_OPTS="$DEFAULT_JVM_OPTS $JAVA_OPTS $GRADLE_OPTS"

# Use the maximum available, or set MAX_FD != -1 to use that value.
if [ "$MAX_FD" = "maximum" -o "$MAX_FD" = "max" ]; then
    # Increase the maximum file descriptors if we can.
    if ! ulimit -n -H 2>/dev/null | grep -q 'unlimited' ; then
        if [ "$darwin" = "true" ]; then
            ulimit -n 10240
        else
            ulimit -n `ulimit -n -H`
        fi
    fi
fi

if [ "$MAX_FD" != -1 ]; then
    # Increase the maximum file descriptors if we can.
    if [ "$darwin" = "true" ]; then
        ulimit -n $MAX_FD
    else
        ulimit -n $MAX_FD
    fi
fi

# Add the launcher to the CLASSPATH.
CLASSPATH="$gradle_launcher_jar"

# Prepend the GRADLE_HOME paths to the CLASSPATH
if [ -n "$GRADLE_HOME" ]; then
    CLASSPATH="$CLASSPATH:$GRADLE_HOME/lib/gradle-launcher-*.jar"
fi

# If the distribution is not installed, download and install it
if [ ! -d "$distribution_dir" ]; then
    echo "Downloading $distributionUrl"
    mkdir -p "$distribution_dir"
    if [ `command -v curl` ]; then
        curl --progress-bar --location --output "$distribution_zip_file" "$distributionUrl"
    elif [ `command -v wget` ]; then
        wget --progress=bar:force -O "$distribution_zip_file" "$distributionUrl"
    else
        echo "Neither curl nor wget is available."
        exit 1
    fi
    echo "Unzipping $distribution_zip_file to $distribution_dir"
    unzip -q "$distribution_zip_file" -d "$distribution_dir"
fi

# Set the GRADLE_HOME
GRADLE_HOME="$distribution_dir/$distribution_name"

# Execute Gradle
exec java $JVM_OPTS -classpath "$CLASSPATH" org.gradle.wrapper.GradleWrapperMain "$@"
