// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title EncryptedTodoList
 * @dev A privacy-preserving todo list where tasks are stored encrypted
 * Only the task owner can decrypt and view their tasks
 * Demonstrates practical FHE application in daily task management
 */
contract EncryptedTodoList {
    // Structure for encrypted todo items
    struct EncryptedTask {
        bytes encryptedData;    // The encrypted task content
        bytes32 taskId;         // Unique identifier for the task
        uint256 createdAt;      // Timestamp
        bool isCompleted;       // Completion status (encrypted)
    }
    
    // Mapping from user address to their encrypted tasks
    mapping(address => EncryptedTask[]) private userTasks;
    
    // Mapping to track task completion status (encrypted)
    mapping(address => mapping(bytes32 => bytes)) private encryptedStatus;
    
    // Events for transparency (without revealing content)
    event TaskAdded(address indexed user, bytes32 indexed taskId);
    event TaskCompleted(address indexed user, bytes32 indexed taskId);
    event TaskDeleted(address indexed user, bytes32 indexed taskId);
    
    /**
     * @dev Add a new encrypted task to the user's list
     * @param _encryptedData The encrypted task content
     * @param _taskId Unique identifier for the task
     */
    function addEncryptedTask(bytes memory _encryptedData, bytes32 _taskId) public {
        require(_encryptedData.length > 0, "Encrypted data cannot be empty");
        require(_taskId != bytes32(0), "Invalid task ID");
        
        EncryptedTask memory newTask = EncryptedTask({
            encryptedData: _encryptedData,
            taskId: _taskId,
            createdAt: block.timestamp,
            isCompleted: false
        });
        
        userTasks[msg.sender].push(newTask);
        
        emit TaskAdded(msg.sender, _taskId);
    }
    
    /**
     * @dev Mark a task as completed (with encrypted status)
     * @param _taskId The task identifier
     * @param _encryptedStatus Encrypted completion status
     */
    function completeTask(bytes32 _taskId, bytes memory _encryptedStatus) public {
        require(_taskId != bytes32(0), "Invalid task ID");
        require(_encryptedStatus.length > 0, "Encrypted status cannot be empty");
        
        encryptedStatus[msg.sender][_taskId] = _encryptedStatus;
        
        emit TaskCompleted(msg.sender, _taskId);
    }
    
    /**
     * @dev Get all encrypted tasks for the calling user
     * @return Array of encrypted tasks
     */
    function getMyEncryptedTasks() public view returns (EncryptedTask[] memory) {
        return userTasks[msg.sender];
    }
    
    /**
     * @dev Get encrypted completion status for a specific task
     * @param _taskId The task identifier
     * @return Encrypted completion status
     */
    function getEncryptedTaskStatus(bytes32 _taskId) public view returns (bytes memory) {
        return encryptedStatus[msg.sender][_taskId];
    }
    
    /**
     * @dev Get the number of tasks for the calling user
     * @return Number of tasks
     */
    function getTaskCount() public view returns (uint256) {
        return userTasks[msg.sender].length;
    }
    
    /**
     * @dev Delete a task (can only be called by task owner)
     * @param _taskId The task identifier to delete
     */
    function deleteTask(bytes32 _taskId) public {
        EncryptedTask[] storage tasks = userTasks[msg.sender];
        
        for (uint256 i = 0; i < tasks.length; i++) {
            if (tasks[i].taskId == _taskId) {
                // Move the last element to the deleted position
                tasks[i] = tasks[tasks.length - 1];
                tasks.pop();
                emit TaskDeleted(msg.sender, _taskId);
                break;
            }
        }
    }
    
    /**
     * @dev Get task details by index (for iteration)
     * @param _index The index of the task
     * @return Task details
     */
    function getTaskByIndex(uint256 _index) public view returns (EncryptedTask memory) {
        require(_index < userTasks[msg.sender].length, "Index out of bounds");
        return userTasks[msg.sender][_index];
    }
}
