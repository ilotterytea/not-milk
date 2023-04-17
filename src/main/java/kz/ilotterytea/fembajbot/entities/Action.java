package kz.ilotterytea.fembajbot.entities;

import jakarta.persistence.*;
import org.hibernate.annotations.CreationTimestamp;

import java.util.Date;

/**
 * @author ilotterytea
 * @version 1.0
 */
@Entity
@Table(name = "actions")
public class Action {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id;

    @Column(name = "channel_id", insertable = false, updatable = false, nullable = false)
    private Integer channelId;

    @Column(name = "consumer_id", insertable = false, updatable = false, nullable = false)
    private Integer consumerId;

    @ManyToOne
    @JoinColumn(name = "channel_id")
    private Channel channel;

    @ManyToOne
    @JoinColumn(name = "consumer_id")
    private Consumer consumer;

    @Column(name = "command_id", updatable = false, nullable = false)
    private String commandId;

    @Column(updatable = false, nullable = false)
    private String message;

    @CreationTimestamp
    @Temporal(TemporalType.TIMESTAMP)
    @Column(name = "created_at", updatable = false, nullable = false)
    private Date creationTimestamp;

    public Action(Channel channel, Consumer consumer, String commandId, String message) {
        this.channel = channel;
        this.consumer = consumer;
        this.commandId = commandId;
        this.message = message;
    }

    public Action() {}

    public Integer getId() {
        return id;
    }

    public Integer getChannelId() {
        return channelId;
    }

    public Integer getConsumerId() {
        return consumerId;
    }

    public Channel getChannel() {
        return channel;
    }

    public void setChannel(Channel channel) {
        this.channel = channel;
    }

    public Consumer getConsumer() {
        return consumer;
    }

    public void setConsumer(Consumer consumer) {
        this.consumer = consumer;
    }

    public String getCommandId() {
        return commandId;
    }

    public String getMessage() {
        return message;
    }

    public Date getCreationTimestamp() {
        return creationTimestamp;
    }
}
