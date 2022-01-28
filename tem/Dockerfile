FROM ioen/rsm:a6ac043

RUN apt-get update && apt-get install -y curl net-tools iproute2
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash
RUN apt-get install -y nodejs

RUN mkdir /tem

WORKDIR /tem

RUN mkdir /tem/databases
RUN mkdir /tem/express-relayer

COPY ./dna/workdir/happ/tem.happ /tem
COPY ./scripts/ /tem
COPY ./express-relayer/ /tem/express-relayer

RUN cd /tem/express-relayer && npm install

CMD sh /tem/execute.sh