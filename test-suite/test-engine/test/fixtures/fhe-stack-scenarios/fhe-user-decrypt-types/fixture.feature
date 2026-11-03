Feature: FHE user decryption of every type
  Scenario: User decryption of the eight FHE types in one request
    Given alice has deployed the UserDecrypt contract
    Then alice user-decrypts every value in one request
      | getter   | expected                                                                       |
      | xBool    | true                                                                           |
      | xUint8   | 42                                                                             |
      | xUint16  | 16                                                                             |
      | xUint32  | 32                                                                             |
      | xUint64  | 18446744073709551600                                                           |
      | xUint128 | 145275933516363203950142179850024740765                                        |
      | xAddress | 0x8ba1f109551bD432803012645Ac136ddd64DBA72                                     |
      | xUint256 | 74285495974541385002137713624115238327312291047062397922780925695323480915729 |
