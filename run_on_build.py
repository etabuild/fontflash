import subprocess
import platform
from packaging import version
import re
import sys
if platform.system() == "Windows":
    print("correct platform:Windows")
    args = sys.argv
    if args[1] != None:
        if isinstance(version.parse(args[1]),version.Version):
            """    
            package_version_data_raw = subprocess.check_output(["npm", "list"], shell=True)
            package_version_data = re.sub(b"\x1b\\[.+?m", b"", package_version_data_raw).decode().splitlines()
            del package_version_data[0]

            del package_version_data[-1]
            print(re.search(r'\s.*@([0-9]+.)+[0-9]+', package_version_data[1]).group())
            count = 0

            for line in package_version_data:

                line = re.search(r'\s.*@([0-9]+.)+[0-9]+', package_version_data[count]).group()
                package_version_data[count] = re.sub(r'\s', '', line)
                print(package_version_data[count])
                count += 1
            #fbuffer_version_vue = open('src/Components/Version.vue','w')
            #fbuffer_version_vue.write('<template></template><script>export default {name: "Version.vue",data(){return{version:""}}}</script><style scoped></style>')

            subprocess.run(["npm","run","build"],shell=True)
            """
            sys.exit()
            
else:
    print("[ScriptFailed] require Windows platform")
sys.exit(1)