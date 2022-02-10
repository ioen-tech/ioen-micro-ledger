FROM ioen/rsm:a6ac043

RUN apt-get update && apt-get install -y curl net-tools iproute2
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash
RUN apt-get install -y nodejs

RUN mkdir /ledger

WORKDIR /ledger

RUN mkdir /ledger/databases
RUN mkdir /ledger/express-relayer

COPY ./dna/workdir/happ/ledger.happ /ledger
COPY ./scripts/ /ledger
COPY ./express-relayer/ /ledger/express-relayer

RUN cd /ledger/express-relayer && npm install

CMD sh /ledger/execute.sh