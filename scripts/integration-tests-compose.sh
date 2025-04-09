export LOG_LEVEL=debug
make httpz-down
make console-down

make httpz-up
make console-up

make httpz-test-input
status=$?

echo "Localstack:"
docker logs console-aws
echo "Back:"
docker logs console-back
echo "Orchestrator:"
docker logs console-orchestrator
echo "Relayer:"
docker logs console-relayer
exit $status
