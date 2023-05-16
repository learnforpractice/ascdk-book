import os
for root, dirs, files in os.walk('docs'):
    for file in files:
        if file.endswith('.md'):
            cmd = f"notedown {root}/{file} > notebook/{file.replace('.md', '.ipynb')}"
            print(cmd)
            os.system(cmd)
