import czv

data = """fruits,price
apple,2.50
banana,3.00
strawberry,1.50"""

output = czv.row_count(data, False)

print(output)
