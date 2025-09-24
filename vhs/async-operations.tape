# Async Operations Demo - Tracking long-running operations

Output imgimg/async-operations.gif
Set FontSize 16
Set Width 1200
Set Height 700
Set Theme "Monokai Pro"
Set Padding 20
Set TypingSpeed 40ms
Set LineHeight 1.3

Type "# Async Operations with redisctl"
Enter
Sleep 1s
Enter

Type "# All create/update/delete operations support --wait flags"
Enter
Sleep 1s
Enter

Type "# Create a database and wait for completion"
Enter
Type 'redisctl cloud database create --subscription-id 12345 \'
Enter
Type '  --data \'{"name":"production-cache","memoryLimitInGb":2}\' \'
Enter
Type '  --wait'
Enter
Sleep 500ms
Type "# ⣾ Creating database..."
Sleep 500ms
Backspace 25
Type "# ⣽ Creating database..."
Sleep 500ms
Backspace 25
Type "# ⣻ Creating database..."
Sleep 500ms
Backspace 25
Type "# ⢿ Creating database..."
Sleep 500ms
Backspace 25
Type "# ✓ Database created successfully (task_id: 8e7f2a3b)"
Enter
Sleep 2s
Enter

Type "# Custom timeout and polling interval"
Enter
Type 'redisctl enterprise database update 1 \'
Enter
Type '  --data \'{"memory_size":2147483648}\' \'
Enter
Type '  --wait --wait-timeout 600 --wait-interval 5'
Enter
Sleep 500ms
Type "# ⠙ Updating database (checking every 5s)..."
Sleep 500ms
Backspace 45
Type "# ⠹ Updating database (checking every 5s)..."
Sleep 500ms
Backspace 45
Type "# ✓ Database updated successfully"
Enter
Sleep 2s
Enter

Type "# Delete with confirmation and progress tracking"
Enter
Type "redisctl cloud database delete 12345:67890 --wait"
Enter
Sleep 500ms
Type "# ⠋ Deleting database..."
Sleep 500ms
Backspace 24
Type "# ⠙ Deleting database..."
Sleep 500ms
Backspace 24
Type "# ✓ Database deleted successfully"
Enter
Sleep 2s
Enter

Type "# Network connectivity operations also support async"
Enter
Type 'redisctl cloud peering create --subscription-id 12345 \'
Enter
Type '  --data \'{"provider":"AWS","region":"us-east-1"}\' \'
Enter
Type '  --wait'
Enter
Sleep 500ms
Type "# ⠸ Creating VPC peering connection..."
Sleep 500ms
Backspace 39
Type "# ⠼ Creating VPC peering connection..."
Sleep 500ms
Backspace 39
Type "# ✓ VPC peering created (id: vpc-12345)"
Enter
Sleep 2s
