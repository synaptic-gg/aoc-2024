list1 = {}
with open('input.txt','r') as file:
    i = 1
    for line in file:
        list1[f"list{i}"] = f"{line.replace(' ',',').replace('\n','')}"
        i += 1

with open('input_san.txt','w') as file:
    for i in list1:
        file.write(f"{i} = [{list1[i]}] \n")
    file.close()



