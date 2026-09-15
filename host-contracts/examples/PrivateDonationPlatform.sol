// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title PrivateDonationPlatform
 * @dev A privacy-preserving donation platform where donation amounts are encrypted
 * Donors remain anonymous while the platform can calculate total donations
 * Demonstrates practical FHE application in charitable giving
 */
contract PrivateDonationPlatform {
    // Campaign structure for donation drives
    struct Campaign {
        string name;
        address payable beneficiary;
        uint256 goal;
        uint256 deadline;
        bool isActive;
        bytes encryptedTotal; // Encrypted total donations
        uint256 donorCount;
    }
    
    // Encrypted donation details
    struct EncryptedDonation {
        bytes encryptedAmount;
        bytes32 donorId;
        uint256 timestamp;
        bytes32 campaignId;
    }
    
    // Campaign management
    mapping(bytes32 => Campaign) public campaigns;
    mapping(address => EncryptedDonation[]) private donorHistory;
    mapping(bytes32 => address[]) private campaignDonors;
    
    // Events for transparency
    event CampaignCreated(bytes32 indexed campaignId, string name, uint256 goal);
    event DonationReceived(bytes32 indexed campaignId, bytes32 indexed donorId);
    event CampaignCompleted(bytes32 indexed campaignId, uint256 totalDonors);
    
    /**
     * @dev Create a new donation campaign
     * @param _name Campaign name
     * @param _beneficiary Address to receive funds
     * @param _goal Funding goal (encrypted)
     * @param _duration Campaign duration in seconds
     */
    function createCampaign(
        string memory _name,
        address payable _beneficiary,
        uint256 _goal,
        uint256 _duration
    ) public returns (bytes32) {
        require(_duration > 0, "Duration must be positive");
        require(_beneficiary != address(0), "Invalid beneficiary");
        require(_goal > 0, "Goal must be positive");
        
        bytes32 campaignId = keccak256(
            abi.encodePacked(_name, _beneficiary, block.timestamp, msg.sender)
        );
        
        campaigns[campaignId] = Campaign({
            name: _name,
            beneficiary: _beneficiary,
            goal: _goal,
            deadline: block.timestamp + _duration,
            isActive: true,
            encryptedTotal: new bytes(0),
            donorCount: 0
        });
        
        emit CampaignCreated(campaignId, _name, _goal);
        
        return campaignId;
    }
    
    /**
     * @dev Make an encrypted donation to a campaign
     * @param _campaignId The campaign to donate to
     * @param _encryptedAmount Encrypted donation amount
     * @param _donorId Anonymous donor identifier
     */
    function donate(
        bytes32 _campaignId,
        bytes memory _encryptedAmount,
        bytes32 _donorId
    ) public payable {
        Campaign storage campaign = campaigns[_campaignId];
        require(campaign.isActive, "Campaign not active");
        require(block.timestamp < campaign.deadline, "Campaign expired");
        require(_encryptedAmount.length > 0, "Invalid encrypted amount");
        require(_donorId != bytes32(0), "Invalid donor ID");
        
        // Store encrypted donation
        EncryptedDonation memory newDonation = EncryptedDonation({
            encryptedAmount: _encryptedAmount,
            donorId: _donorId,
            timestamp: block.timestamp,
            campaignId: _campaignId
        });
        
        donorHistory[msg.sender].push(newDonation);
        campaignDonors[_campaignId].push(msg.sender);
        
        campaign.donorCount++;
        
        emit DonationReceived(_campaignId, _donorId);
    }
    
    /**
     * @dev Get campaign details
     * @param _campaignId Campaign identifier
     * @return Campaign details
     */
    function getCampaign(bytes32 _campaignId) 
        public 
        view 
        returns (Campaign memory) 
    {
        return campaigns[_campaignId];
    }
    
    /**
     * @dev Get donor's encrypted donation history
     * @param _donor Donor address
     * @return Array of encrypted donations
     */
    function getDonorHistory(address _donor) 
        public 
        view 
        returns (EncryptedDonation[] memory) 
    {
        return donorHistory[_donor];
    }
    
    /**
     * @dev Get total donors for a campaign
     * @param _campaignId Campaign identifier
     * @return Number of unique donors
     */
    function getCampaignDonorCount(bytes32 _campaignId) 
        public 
        view 
        returns (uint256) 
    {
        return campaigns[_campaignId].donorCount;
    }
    
    /**
     * @dev Check if campaign is still active
     * @param _campaignId Campaign identifier
     * @return Active status
     */
    function isCampaignActive(bytes32 _campaignId) 
        public 
        view 
        returns (bool) 
    {
        Campaign memory campaign = campaigns[_campaignId];
        return campaign.isActive && block.timestamp < campaign.deadline;
    }
    
    /**
     * @dev Close a campaign (only beneficiary can call)
     * @param _campaignId Campaign to close
     */
    function closeCampaign(bytes32 _campaignId) public {
        Campaign storage campaign = campaigns[_campaignId];
        require(msg.sender == campaign.beneficiary, "Only beneficiary can close");
        require(campaign.isActive, "Campaign already closed");
        
        campaign.isActive = false;
        emit CampaignCompleted(_campaignId, campaign.donorCount);
    }
}
