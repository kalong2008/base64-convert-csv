convert base64 data/audio wav to wav file

get the current complie file location, and save the wav in the same folder

please complie first

it will not work when run ./base64-convert-csv

it will also not work when run in visual studio code

the raw.csv is the sample csv file

the program will read raw.csv, so make sure the is name as "raw.csv"

please download the file from the google sheet (sheet3) as csv, then rename to "raw.csv"

since each cell will have maximum length, cannot store all of the base64 string, therefore, the base64 string will split into several row. 

the rust will combine the each base64 into one row, then convert it to wav

the csv file do not have header, and only have one column