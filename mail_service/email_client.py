import grpc

import demo_pb2
import demo_pb2_grpc

from log import getJSONLogger
logger = getJSONLogger('emailservice-client')

def send_confirmation_email(email, order):
  channel = grpc.insecure_channel('[::]:8080')
  stub = demo_pb2_grpc.EmailServiceStub(channel)
  try:
    response = stub.SendOrderConfirmation(demo_pb2.SendOrderConfirmationRequest(
      email = email,
      order = order
    ))
    logger.info('Request sent.')
  except grpc.RpcError as err:
    logger.error(err.details())
    logger.error('{}, {}'.format(err.code().name, err.code().value))

if __name__ == '__main__':
  logger.info('Client for email service.')