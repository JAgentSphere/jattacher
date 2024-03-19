## jattacher

A utility to diagnostics JVM processes using the Dynamic Attach mechanism.

It is a Rust implementation of
the [HotSpotVirtualMachine](https://github.com/openjdk/jdk8u/blob/master/jdk/src/share/classes/sun/tools/attach/HotSpotVirtualMachine.java)
and [OpenJ9VirtualMachine](https://github.com/eclipse-openj9/openj9/blob/master/jcl/src/jdk.attach/share/classes/com/ibm/tools/attach/attacher/OpenJ9VirtualMachine.java)
from the tools.jar of the JDK. However, No installed JDK required, works with just JRE.

### Features

- [ ] `jattacher jps` —— List all Java processes along with comprehensive process information, going beyond the default
  JPS output
  provided by OpenJ9 and HotSpot.
- [ ] `jattacher jps --json` —— Output data in JSON format.
- [ ] `jattacher jps --dashboard` —— Display Java process information in real time.

### HotSpot Support

> [sun/tools](https://github.com/openjdk/jdk8u/tree/master/jdk/src/share/classes/sun/tools)

- [ ] [`jps`](https://github.com/openjdk/jdk8u/blob/master/jdk/src/share/classes/sun/tools/jps/Arguments.java)
- [ ] [`jcmd`](https://github.com/openjdk/jdk8u/blob/master/jdk/src/share/classes/sun/tools/jcmd/Arguments.java)
- [ ] [`jmap`](https://github.com/openjdk/jdk8u/blob/master/jdk/src/share/classes/sun/tools/jmap/JMap.java)
- [ ] [`jstack`](https://github.com/openjdk/jdk8u/blob/master/jdk/src/share/classes/sun/tools/jstack/JStack.java)
- [ ] [`jstat`](https://github.com/openjdk/jdk8u/blob/master/jdk/src/share/classes/sun/tools/jstat/Arguments.java)
- [ ] [`jinfo`](https://github.com/openjdk/jdk8u/blob/master/jdk/src/share/classes/sun/tools/jinfo/JInfo.java)

### OpenJ9 Support

> [openj9/tools/attach/diagnostics/tools](https://github.com/eclipse-openj9/openj9/tree/master/jcl/src/jdk.jcmd/share/classes/openj9/tools/attach/diagnostics/tools)

- [ ] [`jps`](https://github.com/eclipse-openj9/openj9/blob/master/jcl/src/jdk.jcmd/share/classes/openj9/tools/attach/diagnostics/tools/Jps.java)
- [ ] [`jcmd`](https://github.com/eclipse-openj9/openj9/blob/master/jcl/src/jdk.jcmd/share/classes/openj9/tools/attach/diagnostics/tools/Jcmd.java)
- [ ] [`jmap`](https://github.com/eclipse-openj9/openj9/blob/master/jcl/src/jdk.jcmd/share/classes/openj9/tools/attach/diagnostics/tools/Jmap.java)
- [ ] [`jstack`](https://github.com/eclipse-openj9/openj9/blob/master/jcl/src/jdk.jcmd/share/classes/openj9/tools/attach/diagnostics/tools/Jstack.java)
- [ ] [`jstat`](https://github.com/eclipse-openj9/openj9/blob/master/jcl/src/jdk.jcmd/share/classes/openj9/tools/attach/diagnostics/tools/Jstat.java)