# Manual Testing Guide for Q CLI History Command

## Overview
This guide provides manual testing steps for the `/history` command implementation since automated testing of the full CLI interaction is limited.

## Prerequisites
1. Build the project: `cargo build`
2. Have some existing Q CLI conversations in your database (use `q chat` in different directories)

## Test Cases

### 1. Basic List Command
```bash
# Test basic listing
./target/debug/chat_cli history list

# Expected: Shows table with recent conversations or "No conversations found"
```

### 2. List with Limit
```bash
# Test with custom limit
./target/debug/chat_cli history list --limit 5

# Expected: Shows at most 5 conversations
```

### 3. List with Path Filter
```bash
# Test path filtering
./target/debug/chat_cli history list --path /home/user
./target/debug/chat_cli history list --path workspace

# Expected: Only shows conversations from paths containing the filter string
```

### 4. Show Specific Conversation
```bash
# First get a conversation ID from the list command
./target/debug/chat_cli history list

# Then show a specific conversation (use actual ID from list)
./target/debug/chat_cli history show f18c31da-422d-43b9-b7b1-bb01fb7c772b

# Test partial ID matching
./target/debug/chat_cli history show f18c31da

# Expected: Shows full conversation transcript with resume instructions
```

### 5. Error Cases
```bash
# Test with non-existent conversation ID
./target/debug/chat_cli history show nonexistent

# Expected: "Conversation with ID 'nonexistent' not found" message
```

### 6. Help Text
```bash
# Test help for main command
./target/debug/chat_cli history --help

# Test help for subcommands
./target/debug/chat_cli history list --help
./target/debug/chat_cli history show --help

# Expected: Proper help text with descriptions and options
```

### 7. Edge Cases
```bash
# Test with zero limit
./target/debug/chat_cli history list --limit 0

# Test with very large limit
./target/debug/chat_cli history list --limit 1000

# Test with empty path filter
./target/debug/chat_cli history list --path ""

# Expected: Should handle gracefully without crashing
```

## Visual Verification

### Table Output Format
The list command should produce a nicely formatted table like:
```
Recent Conversations:
┌─────────────────────┬──────────────────────────────────────┬─────────────────────────────────────┐
│ Date                │ Directory                            │ Preview                             │
├─────────────────────┼──────────────────────────────────────┼─────────────────────────────────────┤
│ 2025-08-03 17:00:00 │ ~/chat-browser                       │ this is a project related to the...│
└─────────────────────┴──────────────────────────────────────┴─────────────────────────────────────┘
```

### Show Command Output
The show command should display:
1. Conversation ID
2. Directory path
3. Message count
4. Resume instructions
5. Full conversation transcript

## Performance Testing

### Large Database
If you have many conversations:
```bash
# Test performance with large result sets
./target/debug/chat_cli history list --limit 100

# Test filtering performance
./target/debug/chat_cli history list --path /very/common/path
```

## Integration Testing

### With Existing Q CLI
1. Create some conversations using the regular Q CLI:
   ```bash
   cd /tmp/test1 && q chat
   # Have a conversation, then exit
   
   cd /tmp/test2 && q chat  
   # Have another conversation, then exit
   ```

2. Test the history command:
   ```bash
   q history list
   # Should show both conversations
   
   q history list --path /tmp
   # Should show both conversations
   
   q history show <id-from-list>
   # Should show full conversation
   ```

3. Test resuming:
   ```bash
   # Follow the resume instructions from the show command
   cd /tmp/test1
   q chat
   # Should resume the previous conversation
   ```

## Regression Testing

### Ensure Existing Functionality Works
1. Regular chat still works: `q chat`
2. Other commands still work: `q login`, `q whoami`, etc.
3. Database integrity maintained (conversations still persist)

## Error Scenarios

### Database Issues
1. Test with corrupted database entries (if possible)
2. Test with permission issues on database file
3. Test with missing database

### Invalid Input
1. Test with invalid conversation IDs
2. Test with negative limits
3. Test with very long path filters

## Expected Behavior Summary

✅ **Should Work:**
- List conversations with proper formatting
- Filter by path substring
- Limit results appropriately
- Show full conversations by ID or partial ID
- Handle empty database gracefully
- Provide helpful error messages
- Display proper help text

❌ **Should Not:**
- Crash on any input
- Show corrupted or malformed data
- Expose sensitive information
- Break existing Q CLI functionality
- Have memory leaks or performance issues

## Reporting Issues

If any test fails:
1. Note the exact command used
2. Record the error message or unexpected output
3. Check if the issue is reproducible
4. Note your environment (OS, Rust version, etc.)
5. Include relevant logs if available

## Cleanup

After testing, you may want to:
```bash
# Clean up test conversations if needed
# (Note: There's no built-in cleanup command yet)
```
