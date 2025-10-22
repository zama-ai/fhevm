#!/bin/bash

# e2e
#SRC="/Users/alex/src/me/zama-ai/fhevm-with-codegen-dev/test-suite/e2e/codegen"
#DST="../test-suite/e2e/codegen"

# host-contracts
SRC="/Users/alex/src/me/zama-ai/fhevm-with-codegen-dev/library-solidity/codegen"
DST="../library-solidity/codegen"

# diff ../library-solidity/codegen/common.ts ${DST}/common.ts 
# diff ../library-solidity/codegen/generateOverloads.ts ${DST}/generateOverloads.ts
# diff ../library-solidity/codegen/operators.ts ${DST}/operators.ts
# diff ../library-solidity/codegen/overloadTests.ts ${DST}/overloadTests.ts
# diff ../library-solidity/codegen/utils.ts ${DST}/utils.ts
# diff ../library-solidity/codegen/main.ts ${DST}/main.ts
# Wrong types in e2e
# diff ../library-solidity/codegen/types.ts ${DST}/types.ts
# diff ../library-solidity/codegen/templates.ts ${DST}/templates.ts
# Not the same
# diff ../library-solidity/codegen/testgen.ts ${DST}/testgen.ts


diff ${SRC}/common.ts ${DST}/common.ts
diff ${SRC}/generateOverloads.ts ${DST}/generateOverloads.ts
#diff ${SRC}/hcuLimitGenerator.ts ${DST}/hcuLimitGenerator.ts
diff ${SRC}/main.ts ${DST}/main.ts
diff ${SRC}/operators.ts ${DST}/operators.ts

# e2e only
#diff ${SRC}/overLoad.ts ${DST}/overLoad.ts

diff ${SRC}/overloadTests.ts ${DST}/overloadTests.ts

#diff ${SRC}/templates.ts ${DST}/templates.ts
diff ${SRC}/testgen.ts ${DST}/testgen.ts

diff ${SRC}/types.ts ${DST}/types.ts
diff ${SRC}/utils.ts ${DST}/utils.ts

#diff ./src/hcuLimitGenerator.ts ${DST}/hcuLimitGenerator.ts
#diff ./src/templates.ts ../host-contracts/codegen/templates.ts
#