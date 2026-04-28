#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting Taskr Integration Test...${NC}\n"

# 1. Test Adding a Task
echo -e "Testing: Add Task"
taskr --add "Integration Test Task" --priority 5 --note "Testing the full pipeline"
if [ $? -eq 0 ]; then
    echo -e "${GREEN}PASS: Task added successfully${NC}"
else
    echo -e "${RED}FAIL: Could not add task${NC}"
fi

# 2. Test Listing Tasks
echo -e "\nTesting: List Tasks"
LIST_OUTPUT=$(taskr --list)
if [[ $LIST_OUTPUT == *"Integration Test Task"* ]]; then
    echo -e "${GREEN}PASS: Task found in list${NC}"
else
    echo -e "${RED}FAIL: Task not found in list${NC}"
fi

# 3. Test Filtering by Project (Assuming Project ID 1 exists)
echo -e "\nTesting: Project Filter"
# Note: This depends on having a project named 'Default' or whatever your ID 1 is
taskr --list --project-name "Default" > /dev/null
if [ $? -eq 0 ]; then
    echo -e "${GREEN}PASS: Project filter command executed${NC}"
else
    echo -e "${RED}FAIL: Project filter failed${NC}"
fi

# 4. Test Marking as Done
# We'll grab the ID of the task we just created
TASK_ID=$(echo "$LIST_OUTPUT" | grep "Integration Test Task" | awk '{print $1}')

echo -e "\nTesting: Mark Task $TASK_ID as Done"
taskr --done $TASK_ID
if [ $? -eq 0 ]; then
    echo -e "${GREEN}PASS: Task marked as done${NC}"
else
    echo -e "${RED}FAIL: Could not mark task as done${NC}"
fi

# 5. Test Deletion (The big one with the Foreign Keys!)
echo -e "\nTesting: Delete Task $TASK_ID"
taskr --delete $TASK_ID
if [ $? -eq 0 ]; then
    echo -e "${GREEN}PASS: Task $TASK_ID deleted successfully${NC}"
else
    echo -e "${RED}FAIL: Delete failed (Check Foreign Key constraints)${NC}"
fi

# Final Verification
echo -e "\nFinal List State:"
taskr --list

echo -e "\n${GREEN}Test Suite Complete!${NC}"
